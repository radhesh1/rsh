use rsh_test_support::{rsh, pipeline};

#[cfg(feature = "sqlite")]
#[test]
fn can_query_single_table() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            open sample.db
            | query db "select * from strings"
            | where x =~ ell
            | length
        "#
    ));

    assert_eq!(actual.out, "4");
}

#[cfg(feature = "sqlite")]
#[test]
fn invalid_sql_fails() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            open sample.db
            | query db "select *asdfasdf"
        "#
    ));

    assert!(actual.err.contains("syntax error"));
}

#[cfg(feature = "sqlite")]
#[test]
fn invalid_input_fails() {
    let actual = rsh!(
    cwd: "tests/fixtures/formats", pipeline(
        r#"
            "foo" | query db "select * from asdf"
        "#
    ));

    assert!(actual.err.contains("can't convert string"));
}
