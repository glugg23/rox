use std::process::Command;
use std::str;

#[test]
fn class_in_body() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/for/class_in_body.lox",
        ])
        .output()
        .expect("Error while running for/class_in_body()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'class': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn fun_in_body() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/for/fun_in_body.lox",
        ])
        .output()
        .expect("Error while running for/fun_in_body()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'fun': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn scope() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/for/scope.lox",
        ])
        .output()
        .expect("Error while running for/scope()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "0\n-1\nafter\n0\n");
    assert!(result.status.success());
}

#[test]
fn statement_condition() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/for/statement_condition.lox",
        ])
        .output()
        .expect("Error while running for/statement_condition()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 3] Error at '{': Expect expression.\n[line 3] Error at ')': Expect ';' after expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn statement_increment() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/for/statement_increment.lox",
        ])
        .output()
        .expect("Error while running for/statement_increment()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at '{': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn statement_initializer() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/for/statement_initializer.lox",
        ])
        .output()
        .expect("Error while running for/statement_initializer()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 3] Error at '{': Expect expression.\n[line 3] Error at ')': Expect ';' after expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn var_in_body() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/for/var_in_body.lox",
        ])
        .output()
        .expect("Error while running for/var_in_body()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'var': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}
