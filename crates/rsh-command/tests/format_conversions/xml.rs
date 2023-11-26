use rsh_test_support::{rsh, pipeline};

#[test]
fn table_to_xml_text_and_from_xml_text_back_into_table() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            open jt.xml
            | to xml
            | from xml
            | get content
            | where tag == channel
            | get content
            | flatten
            | where tag == item
            | get content
            | flatten
            | where tag == guid
            | get 0.attributes.isPermaLink
        "#
    ));

    assert_eq!(actual.out, "true");
}
