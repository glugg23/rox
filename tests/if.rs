use std::process::Command;
use std::str;

#[test]
fn class_in_else() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/if/class_in_else.lox",
        ])
        .output()
        .expect("Error while running if/class_in_else()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'class': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn class_in_then() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/if/class_in_then.lox",
        ])
        .output()
        .expect("Error while running if/class_in_then()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'class': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn dangling_else() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/if/dangling_else.lox",
        ])
        .output()
        .expect("Error while running if/dangling_else()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "good\n");
    assert!(result.status.success());
}

#[test]
fn test_else() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/if/else.lox",
        ])
        .output()
        .expect("Error while running if/test_else()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "good\ngood\nblock\n"
    );
    assert!(result.status.success());
}

#[test]
fn fun_in_else() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/if/fun_in_else.lox",
        ])
        .output()
        .expect("Error while running if/fun_in_else()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'fun': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn fun_in_then() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/if/fun_in_then.lox",
        ])
        .output()
        .expect("Error while running if/fun_in_then()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'fun': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn test_if() {
    let result = Command::new("cargo")
        .args(&["run", "-q", "--release", "--", "tests/resources/if/if.lox"])
        .output()
        .expect("Error while running if/test_if()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "good\nblock\ntrue\n"
    );
    assert!(result.status.success());
}

#[test]
fn truth() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/if/truth.lox",
        ])
        .output()
        .expect("Error while running if/truth()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "false\nnil\ntrue\n0\nempty\n"
    );
    assert!(result.status.success());
}

#[test]
fn var_in_else() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/if/var_in_else.lox",
        ])
        .output()
        .expect("Error while running if/var_in_else()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'var': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn var_in_then() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/if/var_in_then.lox",
        ])
        .output()
        .expect("Error while running if/var_in_then()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'var': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}
