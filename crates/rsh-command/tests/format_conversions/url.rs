use rsh_test_support::{rsh, pipeline};

#[test]
fn can_encode_and_decode_urlencoding() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
            r#"
                open sample.url
                | url build-query
                | from url
                | get cheese
            "#
    ));

    assert_eq!(actual.out, "comté");
}
