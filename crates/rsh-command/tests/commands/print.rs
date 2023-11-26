use rsh_test_support::rsh;

#[test]
fn print_to_stdout() {
    let actual = rsh!("print 'hello world'");
    assert!(actual.out.contains("hello world"));
    assert!(actual.err.is_empty());
}

#[test]
fn print_to_stderr() {
    let actual = rsh!("print -e 'hello world'");
    assert!(actual.out.is_empty());
    assert!(actual.err.contains("hello world"));
}
