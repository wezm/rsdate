mod args;
mod error;
mod time;

use std::time::Duration;
use std::{env, process};

use ::time::format_description::well_known::Rfc2822;
use ::time::UtcOffset;
use env_logger::Env;
use log::LevelFilter;
use log::{error, info};
use rsntp::{ProtocolError, SntpClient, SynchroniztationError};
use syslog::BasicLogger;

use crate::args::Config;

type Error = Box<dyn std::error::Error>;

const LOG_ENV_VAR: &str = "RSDATE_LOG";

fn main() {
    setup_logging(args::use_syslog());
    let args = match args::parse_args() {
        Ok(Some(config)) => config,
        Ok(None) => process::exit(0),
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    };

    match try_main(args) {
        Ok(0) => {}
        Ok(status) => process::exit(status),
        Err(err) => {
            error!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn try_main(args: Config) -> Result<i32, Error> {
    let mut attempts = 0;
    let mut delay = Duration::from_millis(500);
    let result = loop {
        // We build a new client each time in case new interfaces to bind to become available
        // between attempts
        let mut client = SntpClient::new();
        client.set_timeout(Duration::from_secs(u64::from(args.timeout)));
        let client = client; // discard mutability
        match client.synchronize(&args.ntp_host) {
            Ok(res) => break res,
            Err(SynchroniztationError::ProtocolError(err)) => {
                if let ProtocolError::KissODeath(_) = err {
                    // KoD indicates that the server rejected the request and generally
                    // means that the client should stop sending request to the server.
                    return Err(err.into());
                }
            }
            Err(err) => {
                // Retry in the face of other errors
                if attempts < args.retry || args.retry < 0 {
                    error!(
                        "ntp sync error, retry in {} seconds: {}",
                        delay.as_secs(),
                        err
                    );
                    std::thread::sleep(delay);
                    delay *= 2;
                    attempts += 1;
                } else {
                    return Err(err.into());
                }
            }
        }
    };

    let utc_time = result.datetime().into_offset_date_time()?;
    let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let local_time_str = utc_time.to_offset(local_offset).format(&Rfc2822)?;

    if args.print_time {
        info!("[{}]\t{}", args.ntp_host, local_time_str);
    }

    if args.set_time {
        match time::change_system_time(utc_time) {
            Ok(()) => {
                info!("Local clock set to {}", local_time_str);
                Ok(0)
            }
            Err(_) => Ok(1),
        }
    } else {
        Ok(0)
    }
}

fn setup_logging(use_syslog: bool) {
    if use_syslog {
        // Set up logging to syslog
        let formatter = syslog::Formatter3164 {
            facility: syslog::Facility::LOG_USER,
            hostname: None,
            process: env!("CARGO_PKG_NAME").into(),
            pid: 0,
        };
        let log_res = syslog::unix(formatter)
            .map_err(|err| format!("unable to connect to syslog: {:?}", err))
            .and_then(|logger| {
                log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
                    .map(|()| log::set_max_level(LevelFilter::Info))
                    .map_err(|err| format!("unable to set logger: {:?}", err))
            });
        if let Err(err) = log_res {
            setup_env_logger();
            error!("syslog error, using env logger: {}", err)
        }
    } else {
        setup_env_logger();
    }
}

fn setup_env_logger() {
    // Set up logging to stdio
    match env::var_os(LOG_ENV_VAR) {
        None => env::set_var(LOG_ENV_VAR, "info"),
        Some(_) => {}
    }
    let env = Env::new().filter(LOG_ENV_VAR);
    env_logger::init_from_env(env);
}
