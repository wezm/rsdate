use std::env;

use pico_args::Arguments;

use crate::Error;

pub struct Config {
    pub print_time: bool,
    pub set_time: bool,
    pub use_syslog: bool,
    pub timeout: u16,
    pub ntp_host: String,
}

pub fn parse_args() -> Result<Option<Config>, Error> {
    let mut pargs = Arguments::from_env();
    if pargs.contains(["-V", "--version"]) {
        return print_version();
    } else if pargs.contains(["-h", "--help"]) {
        return print_usage();
    }

    let mut print_time = pargs.contains(["-p", "--print"]);
    let set_time = pargs.contains(["-s", "--set"]);
    if !set_time && !print_time {
        // If neither -p or -s are passed default to -p
        print_time = true;
    }

    Ok(Some(Config {
        print_time,
        set_time,
        use_syslog: pargs.contains(["-l", "--syslog"]),
        timeout: pargs
            .opt_value_from_fn(["-t", "--timeout"], parse_timeout)?
            .unwrap_or(10),
        ntp_host: pargs.free_from_str()?,
    }))
}

fn print_version() -> Result<Option<Config>, Error> {
    println!("{}", version_string());
    Ok(None)
}

fn version_string() -> String {
    format!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}

fn parse_timeout(s: &str) -> Result<u16, &'static str> {
    s.parse().map_err(|_| "not a number")
}

pub fn print_usage() -> Result<Option<Config>, Error> {
    println!(
        "{}

{bin} connects to an ntp server, printing the returned time and/or sets
the system clock.

E.g. {bin} pool.ntp.org

USAGE:
    {bin} [OPTIONS] <HOST>

ARGS:
    <HOST>
        The ntp server to contact.

OPTIONS:
    -h, --help
            Prints help information

    -p, --print
            Print the time returned by the server.

    -s, --set
            Set the system time to the returned time.

    -t, --timeout TIMEOUT
            Sets response timeout in seconds. [default: 10]

    -l, --syslog
            Print messages to syslog.

    -V, --version
            Prints version information

AUTHOR
    {}

SEE ALSO
    Project source code: https://github.com/wezm/rsdate ",
        version_string(),
        env!("CARGO_PKG_AUTHORS"),
        bin = env!("CARGO_PKG_NAME")
    );
    Ok(None)
}
