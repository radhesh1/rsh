use rsh_test_support::rsh;

#[test]
fn can_round_very_large_numbers() {
    let actual = rsh!("18.1372544780074142289927665486772012345 | math round");

    assert_eq!(actual.out, "18")
}

#[test]
fn can_round_very_large_numbers_with_precision() {
    let actual = rsh!("18.13725447800741422899276654867720121457878988 | math round --precision 10");

    assert_eq!(actual.out, "18.137254478")
}

#[test]
fn can_round_integer_with_negative_precision() {
    let actual = rsh!("123 | math round --precision -1");

    assert_eq!(actual.out, "120")
}

#[test]
fn can_round_float_with_negative_precision() {
    let actual = rsh!("123.3 | math round --precision -1");

    assert_eq!(actual.out, "120")
}

#[test]
fn fails_with_wrong_input_type() {
    let actual = rsh!("\"not_a_number\" | math round");

    assert!(actual.err.contains("command doesn't support"))
}
