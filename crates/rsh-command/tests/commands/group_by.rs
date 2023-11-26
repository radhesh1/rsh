use rsh_test_support::fs::Stub::FileWithContentToBeTrimmed;
use rsh_test_support::playground::Playground;
use rsh_test_support::{rsh, pipeline};

#[test]
fn groups() {
    Playground::setup("group_by_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "los_tres_caballeros.csv",
            r#"
                first_name,last_name,rusty_at,type
                Andrés,Robalino,10/11/2013,A
                JT,Turner,10/12/2013,B
                Yehuda,Katz,10/11/2013,A
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                open los_tres_caballeros.csv
                | group-by rusty_at
                | get "10/11/2013"
                | length
            "#
        ));

        assert_eq!(actual.out, "2");
    })
}

#[test]
fn errors_if_given_unknown_column_name() {
    Playground::setup("group_by_test_2", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "los_tres_caballeros.json",
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
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                open los_tres_caballeros.json
                | group-by {|| get rsh.releases.version }
            "#
        ));

        assert!(actual
            .err
            .contains("requires a table with one value for grouping"));
    })
}

#[test]
fn errors_if_column_not_found() {
    Playground::setup("group_by_test_3", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "los_tres_caballeros.csv",
            r#"
                first_name,last_name,rusty_at,type
                Andrés,Robalino,10/11/2013,A
                JT,Turner,10/12/2013,B
                Yehuda,Katz,10/11/2013,A
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open los_tres_caballeros.csv
                | group-by ttype
            "
        ));

        assert!(actual.err.contains("did you mean 'type'"),);
    })
}

#[test]
fn group_by_on_empty_list_returns_empty_record() {
    let actual = rsh!("[[a b]; [1 2]] | where false | group-by a");
    assert!(actual.err.is_empty());
    assert!(actual.out.contains("empty record"));
}

#[test]
fn optional_cell_path_works() {
    let actual = rsh!("[{foo: 123}, {foo: 234}, {bar: 345}] | group-by foo? | to nuon");
    let expected = r#"{"123": [[foo]; [123]], "234": [[foo]; [234]]}"#;
    assert_eq!(actual.out, expected)
}
