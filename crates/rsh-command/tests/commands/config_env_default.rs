use rsh_test_support::rsh;

#[test]
fn print_config_env_default_to_stdout() {
    let actual = rsh!("config env --default");
    assert_eq!(
        actual.out,
        rsh_utils::get_default_env().replace(['\n', '\r'], "")
    );
    assert!(actual.err.is_empty());
}
