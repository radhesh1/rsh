use rsh_test_support::{rsh, pipeline};

#[test]
fn binary_skip() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            open sample_data.ods --raw |
            skip 2 |
            take 2 |
            into int --endian big
        "#
    ));

    assert_eq!(actual.out, "772");
}

#[test]
fn fail_on_non_iterator() {
    let actual = rsh!("1 | skip 2");

    assert!(actual.err.contains("command doesn't support"));
}
