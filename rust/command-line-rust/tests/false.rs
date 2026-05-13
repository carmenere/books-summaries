use assert_cmd::Command;

#[test]
fn false_works() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}