use std::process::Command;
use std::str;

#[test]
fn and() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/logical_operator/and.lox",
        ])
        .output()
        .expect("Error while running logical_operator/and()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "false\n1\nfalse\ntrue\n3\ntrue\nfalse\n"
    );
    assert!(result.status.success());
}

#[test]
fn and_truth() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/logical_operator/and_truth.lox",
        ])
        .output()
        .expect("Error while running logical_operator/and_truth()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "false\nnil\nok\nok\nok\n"
    );
    assert!(result.status.success());
}

#[test]
fn or() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/logical_operator/or.lox",
        ])
        .output()
        .expect("Error while running logical_operator/or()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "1\n1\ntrue\nfalse\nfalse\nfalse\ntrue\n"
    );
    assert!(result.status.success());
}

#[test]
fn or_truth() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/logical_operator/or_truth.lox",
        ])
        .output()
        .expect("Error while running logical_operator/or_truth()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "ok\nok\ntrue\n0\ns\n"
    );
    assert!(result.status.success());
}
