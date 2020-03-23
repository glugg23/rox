use std::process::Command;
use std::str;

#[test]
fn error_after_multiline() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/string/error_after_multiline.lox",
        ])
        .output()
        .expect("Error while running string/error_after_multiline()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Undefined variable 'err'.\n[line 7] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn literals() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/string/literals.lox",
        ])
        .output()
        .expect("Error while running string/literals()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "()\na string\nA~¶Þॐஃ\n"
    );
    assert!(result.status.success());
}

#[test]
fn multiline() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/string/multiline.lox",
        ])
        .output()
        .expect("Error while running string/multiline()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "1\n2\n3\n");
    assert!(result.status.success());
}

#[test]
fn unterminated() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/string/unterminated.lox",
        ])
        .output()
        .expect("Error while running string/unterminated()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at '\"this string has no close quote': Unterminated string.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}
