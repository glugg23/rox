use std::process::Command;
use std::str;

#[test]
#[ignore] //Ignore for now, will remove and replace later
fn evaluate() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/expressions/evaluate.lox",
        ])
        .output()
        .expect("Error while running expressions/evaluate()");

    assert!(result.status.success());
    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "2\n");
}
