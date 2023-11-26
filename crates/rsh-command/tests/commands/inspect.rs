use rsh_test_support::rsh;

#[test]
fn inspect_with_empty_pipeline() {
    let actual = rsh!("inspect");
    assert!(actual.err.contains("no input value was piped in"));
}
