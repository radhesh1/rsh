use rsh_test_support::rsh;

#[test]
fn print_config_rsh_default_to_stdout() {
    let actual = rsh!("config rsh --default");
    assert_eq!(
        actual.out,
        rsh_utils::get_default_config().replace(['\n', '\r'], "")
    );
    assert!(actual.err.is_empty());
}
