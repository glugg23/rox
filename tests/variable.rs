use std::process::Command;
use std::str;

#[test]
fn redeclare_global() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/redeclare_global.lox",
        ])
        .output()
        .expect("Error while running variable/redeclare_global()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "nil\n");
    assert!(result.status.success());
}

#[test]
fn redefine_global() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/redefine_global.lox",
        ])
        .output()
        .expect("Error while running variable/redefine_global()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "2\n");
    assert!(result.status.success());
}

#[test]
fn undefined_global() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/undefined_global.lox",
        ])
        .output()
        .expect("Error while running variable/undefined_global()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Undefined variable 'notDefined'.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn uninitialized() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/uninitialized.lox",
        ])
        .output()
        .expect("Error while running variable/uninitialized()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "nil\n");
    assert!(result.status.success());
}

#[test]
fn use_false_as_var() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/use_false_as_var.lox",
        ])
        .output()
        .expect("Error while running variable/use_false_as_var()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'false': Expect variable name.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn use_global_in_initializer() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/use_global_in_initializer.lox",
        ])
        .output()
        .expect("Error while running variable/use_global_in_initializer()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "value\n");
    assert!(result.status.success());
}

#[test]
fn use_nil_as_var() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/use_nil_as_var.lox",
        ])
        .output()
        .expect("Error while running variable/use_nil_as_var()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'nil': Expect variable name.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn use_this_as_var() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/use_this_as_var.lox",
        ])
        .output()
        .expect("Error while running variable/use_this_as_var()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'this': Expect variable name.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}
