<h1 align="center">
  🦀📅<br>
  rsdate
</h1>

<div align="center">
  <strong>rsdate connects to an ntp server, printing the returned time and/or
  sets the system clock.</strong>
</div>

<br>

<div align="center">
  <a href="https://cirrus-ci.com/github/wezm/rsdate">
    <img src="https://api.cirrus-ci.com/github/wezm/rsdate.svg" alt="Build Status"></a>
  <a href="https://crates.io/crates/rsdate">
    <img src="https://img.shields.io/crates/v/rsdate.svg" alt="Version">
  </a>
  <img src="https://img.shields.io/crates/l/rsdate.svg" alt="License">
</div>

<br>

Example:

    $ rsdate pool.ntp.org
    [2021-10-11T05:02:32Z INFO  rsdate] [pool.ntp.org]	Mon, 11 Oct 2021 15:02:32 +1000

Download
--------

Pre-compiled binaries are available for a number of platforms.

* [FreeBSD 13 amd64](https://releases.wezm.net/rsdate/0.3.0/rsdate-0.3.0-amd64-unknown-freebsd.tar.gz)
* [Linux x86\_64](https://releases.wezm.net/rsdate/0.3.0/rsdate-0.3.0-x86_64-unknown-linux-musl.tar.gz)
* [MacOS x86\_64](https://releases.wezm.net/rsdate/0.3.0/rsdate-0.3.0-x86_64-apple-darwin.tar.gz)
<!-- * [Windows x86\_64](https://releases.wezm.net/rsdate/0.3.0/rsdate-0.3.0-x86_64-pc-windows-msvc.zip) -->

Example to download and extract a binary:

    curl https://releases.wezm.net/rsdate/0.3.0/rsdate-0.3.0-x86_64-unknown-linux-musl.tar.gz | tar zxf -

Usage
-----

```
USAGE:
    rsdate [OPTIONS] <HOST>

ARGS:
    <HOST>
        The ntp server to contact.

OPTIONS:
    -h, --help
            Prints help information

    -p, --print
            Print the time returned by the server.

    -r, --retry NUMBER
            If retrieving the time fails retry NUMBER times. Retries are made
            at 1, 2, 4, 8, 16, etc. seconds.  A value of 0 disables retry
            (default). A negative value retries forever.

    -s, --set
            Set the system time to the returned time.

    -t, --timeout TIMEOUT
            Sets response timeout in seconds. [default: 10]

    -l, --syslog
            Print messages to syslog.

    -V, --version
            Prints version information
```

Build from Source
-----------------

**Minimum Supported Rust Version:** 1.53.0

`rsdate` is implemented in Rust. See the Rust website for [instructions on
installing the toolchain][rustup].

### From Git Checkout or Release Tarball

Build the binary with `cargo build --release --locked`. The binary will be in
`target/release/rsdate`.

### From crates.io

`cargo install rsdate`

Credits
-------

This tool is inspired by [rdate](https://www.aelius.com/njh/rdate/).

Licence
-------

This project is dual licenced under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/wezm/rsdate/blob/master/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/wezm/rsdate/blob/master/LICENSE-MIT))

at your option.

[rustup]: https://www.rust-lang.org/tools/install
