mod cap;
mod preach;
mod tru;
mod yo;

use assert_cmd::Command;

/// Runs `bin` with `args`, asserts it succeeded, and returns its stdout.
pub fn run_stdout(bin: &str, args: &[&str]) -> String {
    let output = Command::cargo_bin(bin)
        .unwrap()
        .args(args)
        .output()
        .expect("failed to run command");
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("invalid utf8")
}
