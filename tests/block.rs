use std::process::Command;
use std::str;

#[test]
fn scope() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/block/scope.lox",
        ])
        .output()
        .expect("Error while running block/scope()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "inner\nouter\n");
    assert!(result.status.success());
}
