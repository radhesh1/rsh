use rsh_test_support::{rsh, pipeline};

#[test]
fn regular_columns() {
    let actual = rsh!(pipeline(
        r#"
            echo [
                [first_name, last_name, rusty_at, type];

                [Andrés Robalino '10/11/2013' A]
                [JT Turner '10/12/2013' B]
                [Yehuda Katz '10/11/2013' A]
            ]
            | reject type first_name
            | columns
            | str join ", "
        "#
    ));

    assert_eq!(actual.out, "last_name, rusty_at");
}

#[test]
fn skip_cell_rejection() {
    let actual = rsh!("[ {a: 1, b: 2,c:txt}, { a:val } ] | reject a | get c?.0");

    assert_eq!(actual.out, "txt");
}

#[test]
fn complex_nested_columns() {
    let actual = rsh!(pipeline(
        r#"
            {
                "rsh": {
                    "committers": [
                        {"name": "Andrés N. Robalino"},
                        {"name": "JT Turner"},
                        {"name": "Yehuda Katz"}
                    ],
                    "releases": [
                        {"version": "0.2"}
                        {"version": "0.8"},
                        {"version": "0.9999999"}
                    ],
                    "0xATYKARNU": [
                        ["Th", "e", " "],
                        ["BIG", " ", "UnO"],
                        ["punto", "cero"]
                    ]
                }
            }
            | reject rsh."0xATYKARNU" rsh.committers
            | get rsh
            | columns
            | str join ", "
        "#,
    ));

    assert_eq!(actual.out, "releases");
}

#[test]
fn ignores_duplicate_columns_rejected() {
    let actual = rsh!(pipeline(
        r#"
            echo [
                ["first name", "last name"];

                [Andrés Robalino]
                [Andrés Jnth]
            ]
            | reject "first name" "first name"
            | columns
            | str join ", "
        "#
    ));

    assert_eq!(actual.out, "last name");
}

#[test]
fn ignores_duplicate_rows_rejected() {
    let actual = rsh!("[[a,b];[1 2] [3 4] [5 6]] | reject 2 2 | to nuon");
    assert_eq!(actual.out, "[[a, b]; [1, 2], [3, 4]]");
}

#[test]
fn reject_record_from_raw_eval() {
    let actual = rsh!(r#"{"a": 3} | reject a | describe"#);

    assert!(actual.out.contains("record"));
}

#[test]
fn reject_table_from_raw_eval() {
    let actual = rsh!(r#"[{"a": 3}] | reject a"#);

    assert!(actual.out.contains("record 0 fields"));
}

#[test]
fn reject_nested_field() {
    let actual = rsh!("{a:{b:3,c:5}} | reject a.b | debug");

    assert_eq!(actual.out, "{a: {c: 5}}");
}

#[test]
fn reject_optional_column() {
    let actual = rsh!("{} | reject foo? | to nuon");
    assert_eq!(actual.out, "{}");

    let actual = rsh!("[{}] | reject foo? | to nuon");
    assert_eq!(actual.out, "[{}]");

    let actual = rsh!("[{} {foo: 2}] | reject foo? | to nuon");
    assert_eq!(actual.out, "[{}, {}]");

    let actual = rsh!("[{foo: 1} {foo: 2}] | reject foo? | to nuon");
    assert_eq!(actual.out, "[{}, {}]");
}

#[test]
fn reject_optional_row() {
    let actual = rsh!("[{foo: 'bar'}] | reject 3? | to nuon");
    assert_eq!(actual.out, "[[foo]; [bar]]");
}

#[test]
fn reject_list_columns() {
    let actual = rsh!("let arg = [type size]; [[name type size];[Cargo.toml file 10mb] [Cargo.lock file 10mb] [src dir 100mb]] | reject $arg | to nuon");
    assert_eq!(actual.out, "[[name]; [Cargo.toml], [Cargo.lock], [src]]");
}

#[test]
fn reject_list_rows() {
    let actual = rsh!("let arg = [2 0]; [[name type size];[Cargo.toml file 10mb] [Cargo.lock file 10mb] [src dir 100mb]] | reject $arg | to nuon");
    assert_eq!(
        actual.out,
        "[[name, type, size]; [Cargo.lock, file, 10000000b]]"
    );
}

#[test]
fn rject_list_mixed() {
    let actual = rsh!("let arg = [ type 2]; [[name type size];[Cargp.toml file 10mb] [ Cargo.lock file 10mb] [src dir 100mb]] | reject $arg | to nuon");
    assert_eq!(
        actual.out,
        "[[name, size]; [Cargp.toml, 10000000b], [Cargo.lock, 10000000b]]"
    );
}

#[test]
fn reject_multiple_rows_ascending() {
    let actual = rsh!("[[a,b];[1 2] [3 4] [5 6]] | reject 1 2 | to nuon");
    assert_eq!(actual.out, "[[a, b]; [1, 2]]");
}

#[test]
fn reject_multiple_rows_descending() {
    let actual = rsh!("[[a,b];[1 2] [3 4] [5 6]] | reject 2 1 | to nuon");
    assert_eq!(actual.out, "[[a, b]; [1, 2]]");
}

#[test]
fn test_ignore_errors_flag() {
    let actual = rsh!("[[a, b]; [1, 2], [3, 4], [5, 6]] | reject 5 -i | to nuon");
    assert_eq!(actual.out, "[[a, b]; [1, 2], [3, 4], [5, 6]]");
}

#[test]
fn test_ignore_errors_flag_var() {
    let actual =
        rsh!("let arg = [5 c]; [[a, b]; [1, 2], [3, 4], [5, 6]] | reject $arg -i | to nuon");
    assert_eq!(actual.out, "[[a, b]; [1, 2], [3, 4], [5, 6]]");
}
