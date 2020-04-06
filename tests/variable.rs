use std::process::Command;
use std::str;

#[test]
fn duplicate_local() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/duplicate_local.lox",
        ])
        .output()
        .expect("Error while running variable/duplicate_local()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 3] Error at 'a': Variable with this name already declared in this scope.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn in_middle_of_block() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/in_middle_of_block.lox",
        ])
        .output()
        .expect("Error while running variable/in_middle_of_block()");

    assert_eq!(
        str::from_utf8(&result.stdout).unwrap(),
        "a\na b\na c\na b d\n"
    );
    assert!(result.status.success());
}

#[test]
fn in_nested_block() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/in_nested_block.lox",
        ])
        .output()
        .expect("Error while running variable/in_nested_block()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "outer\n");
    assert!(result.status.success());
}

#[test]
fn redeclare_global() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/redeclare_global.lox",
        ])
        .output()
        .expect("Error while running variable/redeclare_global()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "nil\n");
    assert!(result.status.success());
}

#[test]
fn redefine_global() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/redefine_global.lox",
        ])
        .output()
        .expect("Error while running variable/redefine_global()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "2\n");
    assert!(result.status.success());
}

#[test]
fn scope_reuse_in_different_blocks() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/scope_reuse_in_different_blocks.lox",
        ])
        .output()
        .expect("Error while running variable/scope_reuse_in_different_blocks()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "first\nsecond\n");
    assert!(result.status.success());
}

#[test]
fn shadow_and_local() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/shadow_and_local.lox",
        ])
        .output()
        .expect("Error while running variable/shadow_and_local()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "outer\ninner\n");
    assert!(result.status.success());
}

#[test]
fn shadow_global() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/shadow_global.lox",
        ])
        .output()
        .expect("Error while running variable/shadow_global()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "shadow\nglobal\n");
    assert!(result.status.success());
}

#[test]
fn shadow_local() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/shadow_local.lox",
        ])
        .output()
        .expect("Error while running variable/shadow_local()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "shadow\nlocal\n");
    assert!(result.status.success());
}

#[test]
fn undefined_global() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/undefined_global.lox",
        ])
        .output()
        .expect("Error while running variable/undefined_global()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Undefined variable 'notDefined'.\n[line 1] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn undefined_local() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/undefined_local.lox",
        ])
        .output()
        .expect("Error while running variable/undefined_local()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "Undefined variable 'notDefined'.\n[line 2] in script\n"
    );
    assert_eq!(result.status.code().unwrap(), 70);
}

#[test]
fn uninitialized() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/uninitialized.lox",
        ])
        .output()
        .expect("Error while running variable/uninitialized()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "nil\n");
    assert!(result.status.success());
}

#[test]
fn unreached_undefined() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/unreached_undefined.lox",
        ])
        .output()
        .expect("Error while running variable/unreached_undefined()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "ok\n");
    assert!(result.status.success());
}

#[test]
fn use_false_as_var() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/use_false_as_var.lox",
        ])
        .output()
        .expect("Error while running variable/use_false_as_var()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'false': Expect variable name.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn use_global_in_initializer() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/use_global_in_initializer.lox",
        ])
        .output()
        .expect("Error while running variable/use_global_in_initializer()");

    assert_eq!(str::from_utf8(&result.stdout).unwrap(), "value\n");
    assert!(result.status.success());
}

#[test]
fn use_local_in_initializer() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/use_local_in_initializer.lox",
        ])
        .output()
        .expect("Error while running variable/use_local_in_initializer()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 3] Error at 'a': Cannot read local variable in its own initializer.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn use_nil_as_var() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/use_nil_as_var.lox",
        ])
        .output()
        .expect("Error while running variable/use_nil_as_var()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'nil': Expect variable name.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}

#[test]
fn use_this_as_var() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "-q",
            "--release",
            "--",
            "tests/resources/variable/use_this_as_var.lox",
        ])
        .output()
        .expect("Error while running variable/use_this_as_var()");

    assert_eq!(
        str::from_utf8(&result.stderr).unwrap(),
        "[line 2] Error at 'this': Expect variable name.\n"
    );
    assert_eq!(result.status.code().unwrap(), 65);
}
