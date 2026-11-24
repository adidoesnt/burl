use crate::{run_failing_test_against_pred, run_successful_test_against_file};
use predicates::prelude::predicate;

const PREACH_FIXTURE_BASE_PATH: &str = "tests/fixtures/preach";

#[test]
fn prints_usage_when_no_args_given() {
    let pred = predicate::str::contains("Usage");
    run_failing_test_against_pred("preach", &[], pred);
}

#[test]
fn preach_one_string_with_newline() {
    let expected_file = format!("{}/{}", PREACH_FIXTURE_BASE_PATH, "sup1.txt");
    run_successful_test_against_file("preach", &["sup man"], &expected_file);
}

#[test]
fn preach_one_string_with_no_newline() {
    let expected_file = format!("{}/{}", PREACH_FIXTURE_BASE_PATH, "sup1.n.txt");
    run_successful_test_against_file("preach", &["sup  man", "-n"], &expected_file);
}

#[test]
fn preach_two_strings_with_newline() {
    let expected_file = format!("{}/{}", PREACH_FIXTURE_BASE_PATH, "sup2.txt");
    run_successful_test_against_file("preach", &["sup", "man"], &expected_file);
}

#[test]
fn preach_two_strings_with_no_newline() {
    let expected_file = format!("{}/{}", PREACH_FIXTURE_BASE_PATH, "sup2.n.txt");
    run_successful_test_against_file("preach", &["sup", "man", "-n"], &expected_file);
}
