use assert_cmd::Command;

#[test]
fn false_works() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}