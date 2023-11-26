use rsh_test_support::rsh;

#[test]
fn test_ansi_shows_error_on_escape() {
    let actual = rsh!(r"ansi --escape \");

    assert!(actual.err.contains("no need for escape characters"))
}

#[test]
fn test_ansi_list_outputs_table() {
    let actual = rsh!("ansi --list | length");

    assert_eq!(actual.out, "424");
}
