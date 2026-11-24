use crate::run_stdout;
use pretty_assertions::assert_eq;

#[test]
fn preach_repeats_with_newline_when_omit_newline_not_set() {
    assert_eq!(run_stdout("preach", &["yo"]), "yo\n");
}

#[test]
fn preach_repeats_without_newline_when_omit_newline_set() {
    assert_eq!(run_stdout("preach", &["-n", "yo"]), "yo");
}
