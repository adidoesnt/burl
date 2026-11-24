mod cap;
mod preach;
mod tru;
mod yo;

use assert_cmd::{Command, assert::{Assert, OutputAssertExt}};
use predicates::Predicate;

fn run_successful_test<T: Predicate<str>>(bin: &str, args: &[&str], pred: T) -> Assert {
    Command::cargo_bin(bin)
        .unwrap()
        .args(args)
        .output()
        .unwrap()
        .assert()
        .success()
        .stdout(pred)
}

fn run_failing_test<T: Predicate<str>>(bin: &str, args: &[&str], pred: T) -> Assert {
    Command::cargo_bin(bin)
        .unwrap()
        .args(args)
        .assert()
        .failure()
        .stderr(pred)
}

