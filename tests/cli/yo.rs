use crate::run_stdout;
use pretty_assertions::assert_eq;

#[test]
fn yo_runs() {
    assert_eq!(run_stdout("yo", &[]), "yo\n");
}
