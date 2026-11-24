use crate::run_successful_test;
use predicates::prelude::predicate;

#[test]
fn yo_runs() {
    let pred = predicate::str::is_match("yo").unwrap();
    run_successful_test("yo", &[], pred);
}
