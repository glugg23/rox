use std::process::Command;
use std::str;

#[test]
fn empty_file() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/empty_file.lox",
        ])
        .output()
        .expect("Error while running misc/empty_file()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "");
    assert!(result.status.success());
}

#[test]
fn precedence() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/precedence.lox",
        ])
        .output()
        .expect("Error while running misc/precedence()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "14\n8\n4\n0\ntrue\ntrue\ntrue\ntrue\n0\n0\n0\n0\n4\n"
    );
    assert!(result.status.success());
}

#[test]
fn unexpected_character() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/unexpected_character.lox",
        ])
        .output()
        .expect("Error while running misc/unexpected_character()");

    //TODO: Fix this test once LeftParen has a precedence of Call
    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 3] Error at '(': Expect ';' after expression.\n[line 3] Error at '|': Unexpected character.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}
