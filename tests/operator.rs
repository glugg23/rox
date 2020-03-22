use std::process::Command;
use std::str;

#[test]
fn add() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/add.lox",
        ])
        .output()
        .expect("Error while running operator/add()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "579\nstring\n");
    assert!(result.status.success());
}

#[test]
fn add_bool_nil() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/add_bool_nil.lox",
        ])
        .output()
        .expect("Error while running operator/add_bool_nil()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be two numbers or two strings.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn add_bool_num() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/add_bool_num.lox",
        ])
        .output()
        .expect("Error while running operator/add_bool_num()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be two numbers or two strings.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn add_bool_string() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/add_bool_string.lox",
        ])
        .output()
        .expect("Error while running operator/add_bool_string()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be two numbers or two strings.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn add_nil_nil() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/add_nil_nil.lox",
        ])
        .output()
        .expect("Error while running operator/add_nil_nil()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be two numbers or two strings.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn add_num_nil() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/add_num_nil.lox",
        ])
        .output()
        .expect("Error while running operator/add_num_nil()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be two numbers or two strings.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn add_string_nil() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/add_string_nil.lox",
        ])
        .output()
        .expect("Error while running operator/add_string_nil()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be two numbers or two strings.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn comparison() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/comparison.lox",
        ])
        .output()
        .expect("Error while running operator/comparison()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "true\nfalse\nfalse\ntrue\ntrue\nfalse\nfalse\nfalse\ntrue\nfalse\ntrue\ntrue\nfalse\nfalse\nfalse\nfalse\ntrue\ntrue\ntrue\ntrue\n"
    );
    assert!(result.status.success());
}

#[test]
fn divide() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/divide.lox",
        ])
        .output()
        .expect("Error while running operator/divide()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "4\n1\n");
    assert!(result.status.success());
}

#[test]
fn divide_nonnum_num() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/divide_nonnum_num.lox",
        ])
        .output()
        .expect("Error while running operator/divide_nonnum_num()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn divide_num_nonnum() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/divide_num_nonnum.lox",
        ])
        .output()
        .expect("Error while running operator/divide_num_nonnum()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn equals() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/equals.lox",
        ])
        .output()
        .expect("Error while running operator/equals()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "true\ntrue\nfalse\ntrue\nfalse\ntrue\nfalse\nfalse\nfalse\nfalse\n"
    );
    assert!(result.status.success());
}

#[test]
fn greater_nonnum_num() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/greater_nonnum_num.lox",
        ])
        .output()
        .expect("Error while running operator/greater_nonnum_num()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn greater_num_nonnum() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/greater_num_nonnum.lox",
        ])
        .output()
        .expect("Error while running operator/greater_num_nonnum()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn greater_or_equal_nonnum_num() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/greater_or_equal_nonnum_num.lox",
        ])
        .output()
        .expect("Error while running operator/greater_or_equal_nonnum_num()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn greater_or_equal_num_nonnum() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/greater_or_equal_num_nonnum.lox",
        ])
        .output()
        .expect("Error while running operator/greater_or_equal_num_nonnum()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn less_nonnum_num() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/less_nonnum_num.lox",
        ])
        .output()
        .expect("Error while running operator/less_nonnum_num()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn less_num_nonnum() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/less_num_nonnum.lox",
        ])
        .output()
        .expect("Error while running operator/less_num_nonnum()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn less_or_equal_nonnum_num() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/less_or_equal_nonnum_num.lox",
        ])
        .output()
        .expect("Error while running operator/less_or_equal_nonnum_num()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn less_or_equal_num_nonnum() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/less_or_equal_num_nonnum.lox",
        ])
        .output()
        .expect("Error while running operator/less_or_equal_num_nonnum()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn multiply() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/multiply.lox",
        ])
        .output()
        .expect("Error while running operator/multiply()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "15\n3.702\n");
    assert!(result.status.success());
}

#[test]
fn multiply_nonnum_num() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/multiply_nonnum_num.lox",
        ])
        .output()
        .expect("Error while running operator/multiply_nonnum_num()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn multiply_num_nonnum() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/multiply_num_nonnum.lox",
        ])
        .output()
        .expect("Error while running operator/multiply_num_nonnum()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn negate() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/negate.lox",
        ])
        .output()
        .expect("Error while running operator/negate()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "-3\n3\n-3\n");
    assert!(result.status.success());
}

#[test]
fn negate_nonnum() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/negate_nonnum.lox",
        ])
        .output()
        .expect("Error while running operator/negate_nonnum()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operand must be a number.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn not_equals() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/not_equals.lox",
        ])
        .output()
        .expect("Error while running operator/not_equals()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "false\nfalse\ntrue\nfalse\ntrue\nfalse\ntrue\ntrue\ntrue\ntrue\n"
    );
    assert!(result.status.success());
}

#[test]
fn subtract() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/subtract.lox",
        ])
        .output()
        .expect("Error while running operator/subtract()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "1\n0\n");
    assert!(result.status.success());
}

#[test]
fn subtract_nonnum_num() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/subtract_nonnum_num.lox",
        ])
        .output()
        .expect("Error while running operator/subtract_nonnum_num()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn subtract_num_nonnum() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/operator/subtract_num_nonnum.lox",
        ])
        .output()
        .expect("Error while running operator/subtract_num_nonnum()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Operands must be numbers.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}
