use rsh_test_support::rsh;

#[test]
fn rolls_4_roll() {
    let actual = rsh!(r#"
        random dice --dice 4 --sides 10 | length
        "#);

    assert_eq!(actual.out, "4");
}
