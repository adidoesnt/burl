use crate::{run_successful_test, run_failing_test};
use predicates::prelude::predicate;

#[test]
fn prints_usage_when_no_args_given() {
    let pred = predicate::str::contains("Usage");
    run_failing_test("preach", &[], pred);
}

#[test]
fn preach_repeats_with_newline_when_omit_newline_not_set() {
    let pred = predicate::str::is_match("yo\n").unwrap();
    run_successful_test("preach", &["yo"], pred);
}

#[test]
fn preach_repeats_without_newline_when_omit_newline_set() {
    let pred = predicate::str::is_match("yo").unwrap();
    run_successful_test("preach", &["yo"], pred);
}
