use assert_cmd::Command;

#[test]
fn yo_runs() {
    let mut cmd = Command::cargo_bin("yo").unwrap();
    cmd.assert().success().stdout("yo\n");
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
