use assert_cmd::Command;

#[test]
fn cap_exits_with_non_zero() {
    // cap always exits with a non-zero code; we expect it to fail.
    Command::cargo_bin("cap").unwrap().assert().failure();
}
