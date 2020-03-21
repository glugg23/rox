use std::process::Command;
use std::str;

#[test]
fn equality() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/bool/equality.lox",
        ])
        .output()
        .expect("Error while running bool/equality()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "true\nfalse\nfalse\ntrue\nfalse\nfalse\nfalse\nfalse\nfalse\nfalse\ntrue\ntrue\nfalse\ntrue\ntrue\ntrue\ntrue\ntrue\n");
    assert!(result.status.success());
}

#[test]
fn not() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/bool/not.lox",
        ])
        .output()
        .expect("Error while running bool/not()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "false\ntrue\ntrue\n"
    );
    assert!(result.status.success());
}
