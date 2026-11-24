use std::fs;

use crate::{run_successful_test, run_failing_test};
use predicates::prelude::predicate;

const PREACH_FIXTURE_BASE_PATH: &str = "tests/fixtures/preach";

#[test]
fn prints_usage_when_no_args_given() {
    let pred = predicate::str::contains("Usage");
    run_failing_test("preach", &[], pred);
}

#[test]
fn preach_one_string_with_newline() {
    let expected = fs::read_to_string(format!("{}/{}", PREACH_FIXTURE_BASE_PATH, "sup1.txt")).expect("failed to read fixture");
    let pred = predicate::str::is_match(expected).unwrap();
    run_successful_test("preach", &["sup man"], pred);
}

#[test]
fn preach_one_string_with_no_newline() { 
    let expected = fs::read_to_string(format!("{}/{}", PREACH_FIXTURE_BASE_PATH, "sup1.n.txt")).expect("failed to read fixture");
    let pred = predicate::str::is_match(expected).unwrap();
    run_successful_test("preach", &["sup  man", "-n"], pred);
}

#[test]
fn preach_two_strings_with_newline() {
    let expected = fs::read_to_string(format!("{}/{}", PREACH_FIXTURE_BASE_PATH, "sup2.txt")).expect("failed to read fixture");
    let pred = predicate::str::is_match(expected).unwrap();
    run_successful_test("preach", &["sup", "man"], pred);
}

#[test]
fn preach_two_strings_with_no_newline() {
    let expected = fs::read_to_string(format!("{}/{}", PREACH_FIXTURE_BASE_PATH, "sup2.n.txt")).expect("failed to read fixture");
    let pred = predicate::str::is_match(expected).unwrap();
    run_successful_test("preach", &["sup", "man", "-n"], pred);
}

