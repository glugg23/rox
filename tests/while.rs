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
            "tests/resources/while/class_in_body.lox",
        ])
        .output()
        .expect("Error while running while/class_in_body()");

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
            "tests/resources/while/fun_in_body.lox",
        ])
        .output()
        .expect("Error while running while/fun_in_body()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'fun': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn syntax() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/while/syntax.lox",
        ])
        .output()
        .expect("Error while running while/syntax()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "1\n2\n3\n0\n1\n2\n"
    );
}

#[test]
fn var_in_body() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/while/var_in_body.lox",
        ])
        .output()
        .expect("Error while running while/var_in_body()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'var': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}
