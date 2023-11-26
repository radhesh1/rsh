use rsh_test_support::rsh;

#[test]
fn length_columns_in_cal_table() {
    let actual = rsh!("cal | columns | length");

    assert_eq!(actual.out, "7");
}

#[test]
fn length_columns_no_rows() {
    let actual = rsh!("echo [] | length");

    assert_eq!(actual.out, "0");
}

#[test]
fn length_fails_on_echo_record() {
    let actual = rsh!("echo {a:1 b:2} | length");

    assert!(actual.err.contains("only_supports_this_input_type"));
}
