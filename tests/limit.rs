use std::process::Command;
use std::str;

#[test]
fn loop_too_large() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/limit/loop_too_large.lox",
        ])
        .output()
        .expect("Error while running limit/loop_too_large()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2351] Error at '}': Loop body too large.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}
