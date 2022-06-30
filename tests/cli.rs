use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

// [2021-10-11T05:30:17Z INFO  rsdate] [pool.ntp.org]	Mon, 11 Oct 2021 15:30:17 +1000
const OUTPUT_PATTERN: &str =
    r"\[INFO  rsdate\] \[pool.ntp.org\]	..., \d{1,2} ... \d{4} \d{2}:\d{2}:\d{2}";

#[test]
fn print() {
    let mut cmd = Command::cargo_bin("rsdate").unwrap();
    cmd.args(&["-p", "pool.ntp.org"]);
    cmd.assert()
        .success()
        .stderr(predicate::str::is_match(OUTPUT_PATTERN).unwrap());
}

#[test]
fn print_default() {
    let mut cmd = Command::cargo_bin("rsdate").unwrap();
    cmd.args(&["pool.ntp.org"]);
    cmd.assert()
        .success()
        .stderr(predicate::str::is_match(OUTPUT_PATTERN).unwrap());
}

#[test]
fn no_args() {
    let mut cmd = Command::cargo_bin("rsdate").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains(
        "free-standing argument is missing",
    ));
}
