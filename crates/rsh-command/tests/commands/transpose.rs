use rsh_test_support::rsh;

#[test]
fn row() {
    let actual = rsh!("[[key value]; [foo 1] [foo 2]] | transpose -r | debug");

    assert!(actual.out.contains("foo: 1"));
}

#[test]
fn row_but_last() {
    let actual = rsh!("[[key value]; [foo 1] [foo 2]] | transpose -r -l | debug");

    assert!(actual.out.contains("foo: 2"));
}

#[test]
fn row_but_all() {
    let actual = rsh!("[[key value]; [foo 1] [foo 2]] | transpose -r -a | debug");

    assert!(actual.out.contains("foo: [1, 2]"));
}
