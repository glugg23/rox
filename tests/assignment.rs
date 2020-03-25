use std::process::Command;
use std::str;

#[test]
fn associativity() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/assignment/associativity.lox",
        ])
        .output()
        .expect("Error while running assignment/associativity()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "c\nc\nc\n");
    assert!(result.status.success());
}

#[test]
fn global() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/assignment/global.lox",
        ])
        .output()
        .expect("Error while running assignment/global()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "before\nafter\narg\narg\n"
    );
    assert!(result.status.success());
}

#[test]
fn grouping() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/assignment/grouping.lox",
        ])
        .output()
        .expect("Error while running assignment/grouping()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at '=': Invalid assignment target.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn infix_operator() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/assignment/infix_operator.lox",
        ])
        .output()
        .expect("Error while running assignment/infix_operator()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 3] Error at '=': Invalid assignment target.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn prefix_operator() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/assignment/prefix_operator.lox",
        ])
        .output()
        .expect("Error while running assignment/prefix_operator()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at '=': Invalid assignment target.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn local() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/assignment/local.lox",
        ])
        .output()
        .expect("Error while running assignment/local()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "before\nafter\narg\narg\n"
    );
    assert!(result.status.success());
}

#[test]
fn syntax() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/assignment/syntax.lox",
        ])
        .output()
        .expect("Error while running assignment/syntax()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "var\nvar\n");
    assert!(result.status.success());
}

#[test]
fn undefined() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/assignment/undefined.lox",
        ])
        .output()
        .expect("Error while running assignment/undefined()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Undefined variable 'unknown'.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}
