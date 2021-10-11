mod args;
mod error;
mod time;

use std::time::Duration;
use std::{env, process};

use chrono::{DateTime, Local};
use env_logger::Env;
use log::LevelFilter;
use log::{error, info};
use rsntp::SntpClient;
use syslog::BasicLogger;

type Error = Box<dyn std::error::Error>;

const LOG_ENV_VAR: &str = "RSDATE_LOG";

fn main() {
    match try_main() {
        Ok(0) => {}
        Ok(status) => process::exit(status),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn try_main() -> Result<i32, Error> {
    let args = match args::parse_args()? {
        Some(config) => config,
        None => return Ok(0),
    };

    setup_logging(args.use_syslog);

    let mut client = SntpClient::new();
    client.set_timeout(Duration::from_secs(u64::from(args.timeout)));
    let client = client; // discard mutability
    let result = client.synchronize(&args.ntp_host)?;

    let local_time: DateTime<Local> = DateTime::from(result.datetime());
    let local_time_str = local_time.to_rfc2822();

    if args.print_time {
        info!("[{}]\t{}", args.ntp_host, local_time_str);
    }

    if args.set_time {
        match time::change_system_time(local_time) {
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
