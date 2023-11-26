use rsh_test_support::fs::Stub::FileWithContentToBeTrimmed;
use rsh_test_support::playground::Playground;
use rsh_test_support::{rsh, pipeline};

#[test]
fn adds_row_data_if_column_missing() {
    Playground::setup("default_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "los_tres_amigos.json",
            r#"
                {
                    "amigos": [
                        {"name":   "Yehuda"},
                        {"name": "JT", "rusty_luck": 0},
                        {"name":   "Andres", "rusty_luck": 0},
                        {"name":"GorbyPuff"}
                    ]
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open los_tres_amigos.json
                | get amigos
                | default 1 rusty_luck
                | where rusty_luck == 1
                | length
            "
        ));

        assert_eq!(actual.out, "2");
    });
}

#[test]
fn default_after_empty_filter() {
    let actual = rsh!("[a b] | where $it == 'c' | last | default 'd'");

    assert_eq!(actual.out, "d");
}
