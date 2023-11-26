use rsh_test_support::rsh;

#[test]
fn can_get_reverse_first() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats",
        "ls | sort-by name | reverse | first | get name | str trim "
    );

    assert_eq!(actual.out, "utf16.ini");
}

#[test]
fn fail_on_non_iterator() {
    let actual = rsh!("1 | reverse");

    assert!(actual.err.contains("command doesn't support"));
}
