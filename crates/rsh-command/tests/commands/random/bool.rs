use rsh_test_support::rsh;

#[test]
fn generates_a_bool() {
    let actual = rsh!("random bool");

    let output = actual.out;
    let is_boolean_output = output == "true" || output == "false";

    assert!(is_boolean_output);
}
