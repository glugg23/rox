use std::process::Command;
use std::str;

#[test]
fn missing_argument() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/print/missing_argument.lox",
        ])
        .output()
        .expect("Error while running print/missing_argument()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at ';': Expect expression.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}
