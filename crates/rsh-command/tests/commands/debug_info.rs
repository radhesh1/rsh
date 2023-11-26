use rsh_test_support::rsh;

#[test]
fn runs_successfully() {
    let actual = rsh!("debug info");
    assert_eq!(actual.err, "");
}
