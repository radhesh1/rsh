use rsh_test_support::rsh;

#[test]
fn convert_back_and_forth() {
    let actual = rsh!(r#"1 | into binary | into int"#);
    assert_eq!(actual.out, "1");
}

#[test]
fn convert_into_int_little_endian() {
    let actual = rsh!(r#"0x[01 00 00 00 00 00 00 00] | into int --endian little"#);
    assert_eq!(actual.out, "1");

    let actual = rsh!(r#"0x[00 00 00 00 00 00 00 01] | into int --endian little"#);
    assert_eq!(actual.out, "72057594037927936");
}

#[test]
fn convert_into_int_big_endian() {
    let actual = rsh!(r#"0x[00 00 00 00 00 00 00 01] | into int --endian big"#);
    assert_eq!(actual.out, "1");

    let actual = rsh!(r#"0x[01 00 00 00 00 00 00 00] | into int --endian big"#);
    assert_eq!(actual.out, "72057594037927936");
}
