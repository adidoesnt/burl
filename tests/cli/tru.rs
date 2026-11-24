use assert_cmd::Command;

#[test]
fn tru_exits_with_zero() {
    Command::cargo_bin("tru").unwrap().assert().success();
}
