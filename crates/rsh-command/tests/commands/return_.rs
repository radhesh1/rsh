use rsh_test_support::{rsh, pipeline};

#[test]
fn early_return_if_true() {
    let actual = rsh!("def foo [x] { if true { return 2 }; $x }; foo 100");

    assert_eq!(actual.out, r#"2"#);
}

#[test]
fn early_return_if_false() {
    let actual = rsh!("def foo [x] { if false { return 2 }; $x }; foo 100");

    assert_eq!(actual.out, r#"100"#);
}

#[test]
fn return_works_in_script_without_def_main() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        "rsh early_return.rsh"
    ));

    assert!(actual.err.is_empty());
}

#[test]
fn return_works_in_script_with_def_main() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats",
        pipeline("rsh early_return_outside_main.rsh")
    );
    assert!(actual.err.is_empty());
}
