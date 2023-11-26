use rsh_test_support::rsh;

#[test]
fn generates_chars_of_specified_length() {
    let actual = rsh!(r#"
        random chars --length 15 | str stats | get chars
        "#);

    let result = actual.out;
    assert_eq!(result, "15");
}
