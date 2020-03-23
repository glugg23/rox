use std::process::Command;
use std::str;

#[test]
fn leading_dot() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/number/leading_dot.lox",
        ])
        .output()
        .expect("Error while running number/leading_dot()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at '.': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn literal() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/number/literal.lox",
        ])
        .output()
        .expect("Error while running number/literal()");

    //Rust prints -0 as 0, meaning this test has been edited
    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "123\n987654\n0\n0\n123.456\n-0.001\n"
    );
    assert!(result.status.success());
}
