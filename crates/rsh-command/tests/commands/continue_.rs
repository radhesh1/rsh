use rsh_test_support::rsh;

#[test]
fn continue_for_loop() {
    let actual = rsh!("for i in 1..10 { if $i == 2 { continue }; print $i }");

    assert_eq!(actual.out, "1345678910");
}
