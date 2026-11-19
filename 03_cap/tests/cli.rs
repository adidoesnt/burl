use assert_cmd::Command;

#[test]
fn exists_with_non_zero() {
    let mut cmd = Command::cargo_bin("cap").unwrap();
    // cap always exits with a non-zero code;
    // we expect it to fail.
    cmd.assert().failure();
}
