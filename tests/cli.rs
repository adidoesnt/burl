use assert_cmd::Command;
use pretty_assertions::assert_eq;

/// Tests for "yo"

#[test]
fn yo_runs() {
    let mut cmd = Command::cargo_bin("yo").unwrap();
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf8");
    assert_eq!(stdout, "yo\n");
}

/// Tests for "tru"

#[test]
fn tru_exits_with_zero() {
    let mut cmd = Command::cargo_bin("tru").unwrap();
    cmd.assert().success();
}

/// Tests for "cap"

#[test]
fn cap_exits_with_non_zero() {
    let mut cmd = Command::cargo_bin("cap").unwrap();
    // cap always exits with a non-zero code;
    // we expect it to fail.
    cmd.assert().failure();
}

/// Tests for "preach"

#[test]
fn preach_repeats_with_newline_when_omit_newline_not_set() {
    let mut cmd = Command::cargo_bin("preach").unwrap();
    let output = cmd.arg("yo").output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf8");
    assert_eq!(stdout, "yo\n");
}

#[test]
fn preach_repeats_without_newline_when_omit_newline_set() {
    let mut cmd = Command::cargo_bin("preach").unwrap();
    let output = cmd.args(["-n", "yo"]).output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf8");
    assert_eq!(stdout, "yo");
}
