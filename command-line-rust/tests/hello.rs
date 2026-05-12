// use std::process::Command;
// #[test]
// fn runs() {
//     let mut cmd = Command::new("./target/debug/hello");
//     let res = cmd.output();
//     assert!(res.is_ok())
// }

use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

#[test]
fn hello_works() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout,"Hello, world!\n");
}