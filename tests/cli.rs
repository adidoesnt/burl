use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn yo_runs() {
    let mut cmd = Command::cargo_bin("yo").unwrap();
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf8");
    assert_eq!(stdout, "yo\n");
}

#[test]
fn tru_exits_with_zero() {
    let mut cmd = Command::cargo_bin("tru").unwrap();
    cmd.assert().success();
}

#[test]
fn cap_exits_with_non_zero() {
    let mut cmd = Command::cargo_bin("cap").unwrap();
    // cap always exits with a non-zero code;
    // we expect it to fail.
    cmd.assert().failure();
}
