use std::process::Command;
use std::str;

#[test]
fn empty() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/block/empty.lox",
        ])
        .output()
        .expect("Error while running block/empty()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "ok\n");
    assert!(result.status.success());
}

#[test]
fn scope() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/block/scope.lox",
        ])
        .output()
        .expect("Error while running block/scope()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "inner\nouter\n");
    assert!(result.status.success());
}
