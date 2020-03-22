use std::process::Command;
use std::str;

#[test]
fn literal() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/nil/literal.lox",
        ])
        .output()
        .expect("Error while running nil/literal()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "nil\n");
    assert!(result.status.success());
}
