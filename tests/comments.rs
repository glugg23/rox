use std::process::Command;
use std::str;

#[test]
fn line_at_eof() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/comments/line_at_eof.lox",
        ])
        .output()
        .expect("Error while running comments/line_at_eof()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "ok\n");
    assert!(result.status.success());
}

#[test]
fn only_line_comment() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/comments/only_line_comment.lox",
        ])
        .output()
        .expect("Error while running comments/only_line_comment()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "");
    assert!(result.status.success());
}

#[test]
fn only_line_comment_and_line() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/comments/only_line_comment_and_line.lox",
        ])
        .output()
        .expect("Error while running comments/only_line_comment_and_line()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "");
    assert!(result.status.success());
}

#[test]
fn unicode() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/comments/unicode.lox",
        ])
        .output()
        .expect("Error while running comments/unicode()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "ok\n");
    assert!(result.status.success());
}
