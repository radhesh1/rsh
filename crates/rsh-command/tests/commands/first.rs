use rsh_test_support::fs::Stub::EmptyFile;
use rsh_test_support::rsh;
use rsh_test_support::playground::Playground;

#[test]
fn gets_first_rows_by_amount() {
    Playground::setup("first_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("los.txt"),
            EmptyFile("tres.txt"),
            EmptyFile("amigos.txt"),
            EmptyFile("arepas.clu"),
        ]);

        let actual = rsh!(cwd: dirs.test(), "ls | first 3 | length");

        assert_eq!(actual.out, "3");
    })
}

#[test]
fn gets_all_rows_if_amount_higher_than_all_rows() {
    Playground::setup("first_test_2", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("los.txt"),
            EmptyFile("tres.txt"),
            EmptyFile("amigos.txt"),
            EmptyFile("arepas.clu"),
        ]);

        let actual = rsh!(
            cwd: dirs.test(), "ls | first 99 | length");

        assert_eq!(actual.out, "4");
    })
}

#[test]
fn gets_first_row_when_no_amount_given() {
    Playground::setup("first_test_3", |dirs, sandbox| {
        sandbox.with_files(vec![EmptyFile("caballeros.txt"), EmptyFile("arepas.clu")]);

        // FIXME: We should probably change first to return a one row table instead of a record here
        let actual = rsh!(cwd: dirs.test(), "ls | first | values | length");

        assert_eq!(actual.out, "4");
    })
}

#[test]
fn gets_first_row_as_list_when_amount_given() {
    let actual = rsh!("[1, 2, 3] | first 1 | describe");

    assert_eq!(actual.out, "list<int> (stream)");
}

#[test]
fn gets_first_bytes() {
    let actual = rsh!("(0x[aa bb cc] | first 2) == 0x[aa bb]");

    assert_eq!(actual.out, "true");
}

#[test]
fn gets_first_byte() {
    let actual = rsh!("0x[aa bb cc] | first");

    assert_eq!(actual.out, "170");
}

#[test]
// covers a situation where `first` used to behave strangely on list<binary> input
fn works_with_binary_list() {
    let actual = rsh!("([0x[01 11]] | first) == 0x[01 11]");

    assert_eq!(actual.out, "true");
}

#[test]
fn errors_on_negative_rows() {
    let actual = rsh!("[1, 2, 3] | first -10");

    assert!(actual.err.contains("use a positive value"));
}

#[test]
fn errors_on_empty_list_when_no_rows_given() {
    let actual = rsh!("[] | first");

    assert!(actual.err.contains("index too large"));
}
