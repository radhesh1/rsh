use rsh_test_support::rsh;

#[test]
fn generates_a_float() {
    let actual = rsh!("random float 42..43");

    // Attention: this relies on the string output
    assert!(actual.out.starts_with("42") || actual.out.starts_with("43"));
    let actual = rsh!("random float 42..43 | describe");

    assert_eq!(actual.out, "float")
}

#[test]
fn generates_55() {
    let actual = rsh!("random float 55..55");

    assert!(actual.out.contains("55"));
}

#[test]
fn generates_0() {
    let actual = rsh!("random float ..<1");

    assert!(actual.out.contains('0'));
}
