use crate::run_successful_test_against_pred;
use predicates::prelude::predicate;

#[test]
fn yo_runs() {
    let pred = predicate::str::is_match("yo").unwrap();
    run_successful_test_against_pred("yo", &[], pred);
}
