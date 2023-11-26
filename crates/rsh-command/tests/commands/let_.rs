use rsh_test_support::rsh;

#[test]
fn let_name_builtin_var() {
    let actual = rsh!("let in = 3");

    assert!(actual
        .err
        .contains("'in' is the name of a builtin RSH variable"));
}

#[test]
fn let_doesnt_mutate() {
    let actual = rsh!("let i = 3; $i = 4");

    assert!(actual.err.contains("immutable"));
}

#[test]
fn let_takes_pipeline() {
    let actual = rsh!(r#"let x = "hello world" | str length; print $x"#);

    assert_eq!(actual.out, "11");
}

#[test]
fn let_pipeline_allows_in() {
    let actual =
        rsh!(r#"def foo [] { let x = $in | str length; print ($x + 10) }; "hello world" | foo"#);

    assert_eq!(actual.out, "21");
}

#[test]
fn mut_takes_pipeline() {
    let actual = rsh!(r#"mut x = "hello world" | str length; print $x"#);

    assert_eq!(actual.out, "11");
}

#[test]
fn mut_pipeline_allows_in() {
    let actual =
        rsh!(r#"def foo [] { mut x = $in | str length; print ($x + 10) }; "hello world" | foo"#);

    assert_eq!(actual.out, "21");
}

#[test]
fn let_pipeline_redirects_internals() {
    let actual = rsh!(r#"let x = echo 'bar'; $x | str length"#);

    assert_eq!(actual.out, "3");
}

#[test]
fn let_pipeline_redirects_externals() {
    let actual = rsh!(r#"let x = rsh --testbin cococo 'bar'; $x | str length"#);

    assert_eq!(actual.out, "3");
}

#[ignore]
#[test]
fn let_with_external_failed() {
    // FIXME: this test hasn't run successfully for a long time. We should
    // bring it back to life at some point.
    let actual = rsh!(r#"let x = rsh --testbin outcome_err "aa"; echo fail"#);

    assert!(!actual.out.contains("fail"));
}
