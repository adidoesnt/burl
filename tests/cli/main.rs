mod cap;
mod preach;
mod tru;
mod yo;

use std::fs;

use assert_cmd::{
    Command,
    assert::{Assert, OutputAssertExt},
};
use predicates::Predicate;
use pretty_assertions::assert_eq;

fn run_successful_test_against_pred<T: Predicate<str>>(
    bin: &str,
    args: &[&str],
    pred: T,
) -> Assert {
    Command::cargo_bin(bin)
        .unwrap()
        .args(args)
        .output()
        .unwrap()
        .assert()
        .success()
        .stdout(pred)
}

fn run_successful_test_against_file(
    bin: &str,
    args: &[&str],
    expected_file: &str,
) {
    let expected = fs::read_to_string(expected_file).expect("Failed to read fixture file");
    let output = String::from_utf8(
        Command::cargo_bin(bin)
            .unwrap()
            .args(args)
            .output()
            .expect("Failed to run command")
            .stdout,
    )
    .expect("Invalid UTF8");

    assert_eq!(output, expected)
}

fn run_failing_test_against_pred<T: Predicate<str>>(bin: &str, args: &[&str], pred: T) -> Assert {
    Command::cargo_bin(bin)
        .unwrap()
        .args(args)
        .assert()
        .failure()
        .stderr(pred)
}
