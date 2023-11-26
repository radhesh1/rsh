use rsh_test_support::{rsh, pipeline};

#[test]
fn test_char_list_outputs_table() {
    let actual = rsh!(pipeline(
        r#"
            char --list | length
        "#
    ));

    assert_eq!(actual.out, "107");
}
