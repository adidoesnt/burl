use assert_cmd::Command;

#[test]
fn exits_with_zero() {
    let mut cmd = Command::cargo_bin("tru").unwrap();
    cmd.assert().success();
}
