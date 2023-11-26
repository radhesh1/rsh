mod collect;
mod into_string;

use rsh_test_support::fs::Stub::FileWithContent;
use rsh_test_support::playground::Playground;
use rsh_test_support::{rsh, pipeline};

#[test]
fn trims() {
    Playground::setup("str_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                    [dependency]
                    name = "rsh "
                "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(),
            "open sample.toml | str trim dependency.name | get dependency.name"
        );

        assert_eq!(actual.out, "rsh");
    })
}

#[test]
fn error_trim_multiple_chars() {
    let actual = rsh!(pipeline(
        r#"
        echo "does it work now?!" | str trim --char "?!"
        "#
    ));

    assert!(actual.err.contains("char"));
}

#[test]
fn capitalizes() {
    Playground::setup("str_test_2", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                    [dependency]
                    name = "rsh"
                "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(),
            "open sample.toml | str capitalize dependency.name | get dependency.name"
        );

        assert_eq!(actual.out, "RSH");
    })
}

#[test]
fn downcases() {
    Playground::setup("str_test_3", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                    [dependency]
                    name = "LIGHT"
                "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(),
            "open sample.toml | str downcase dependency.name | get dependency.name"
        );

        assert_eq!(actual.out, "light");
    })
}

#[test]
fn non_ascii_downcase() {
    let actual = rsh!("'ὈΔΥΣΣΕΎΣ' | str downcase");

    assert_eq!(actual.out, "ὀδυσσεύς");
}

#[test]
fn upcases() {
    Playground::setup("str_test_4", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                    [package]
                    name = "rsh"
                "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(),
            "open sample.toml | str upcase package.name | get package.name"
        );

        assert_eq!(actual.out, "RSH");
    })
}

#[test]
fn non_ascii_upcase() {
    let actual = rsh!("'ὀδυσσεύς' | str upcase");

    assert_eq!(actual.out, "ὈΔΥΣΣΕΎΣ");
}

#[test]
#[ignore = "Playgrounds are not supported in rsh-cmd-extra"]
fn camelcases() {
    Playground::setup("str_test_3", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                    [dependency]
                    name = "THIS_IS_A_TEST"
                "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(),
            "open sample.toml | str camel-case dependency.name | get dependency.name"
        );

        assert_eq!(actual.out, "thisIsATest");
    })
}

#[test]
fn converts_to_int() {
    let actual = rsh!(pipeline(
        r#"
            echo '[{number_as_string: "1"}]'
            | from json
            | into int number_as_string
            | rename number
            | where number == 1
            | get number.0

        "#
    ));

    assert_eq!(actual.out, "1");
}

#[test]
fn converts_to_float() {
    let actual = rsh!(pipeline(
        r#"
            echo "3.1, 0.0415"
            | split row ","
            | into float
            | math sum
        "#
    ));

    assert_eq!(actual.out, "3.1415");
}

#[test]
fn find_and_replaces() {
    Playground::setup("str_test_6", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                     [fortune.teller]
                     phone = "1-800-KATZ"
                 "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                 open sample.toml
                 | str replace KATZ "5289" fortune.teller.phone
                 | get fortune.teller.phone
             "#
        ));

        assert_eq!(actual.out, "1-800-5289");
    })
}

#[test]
fn find_and_replaces_without_passing_field() {
    Playground::setup("str_test_7", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                     [fortune.teller]
                     phone = "1-800-KATZ"
                 "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                 open sample.toml
                 | get fortune.teller.phone
                 | str replace KATZ "5289"
             "#
        ));

        assert_eq!(actual.out, "1-800-5289");
    })
}

#[test]
fn regex_error_in_pattern() {
    Playground::setup("str_test_8", |dirs, _sandbox| {
        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                 'source string'
                 | str replace -r 'source \Ufoo' "destination"
             "#
        ));

        let err = actual.err;
        let expecting_str = "Incorrect value";
        assert!(
            err.contains(expecting_str),
            "Error should contain '{expecting_str}', but was: {err}"
        );
    })
}

#[test]
fn substrings_the_input() {
    Playground::setup("str_test_8", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                     [fortune.teller]
                     phone = "1-800-ROBALINO"
                 "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                 open sample.toml
                 | str substring 6..14 fortune.teller.phone
                 | get fortune.teller.phone
             "#
        ));

        assert_eq!(actual.out, "ROBALINO");
    })
}

#[test]
fn substring_errors_if_start_index_is_greater_than_end_index() {
    Playground::setup("str_test_9", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                     [fortune.teller]
                     phone = "1-800-ROBALINO"
                 "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                 open sample.toml
                 | str substring 6..5 fortune.teller.phone
             "#
        ));

        assert!(actual
            .err
            .contains("End must be greater than or equal to Start"))
    })
}

#[test]
fn substrings_the_input_and_returns_the_string_if_end_index_exceeds_length() {
    Playground::setup("str_test_10", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                     [package]
                     name = "rsh-arepas"
                 "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                 open sample.toml
                 | str substring 0..999 package.name
                 | get package.name
             "#
        ));

        assert_eq!(actual.out, "rsh-arepas");
    })
}

#[test]
fn substrings_the_input_and_returns_blank_if_start_index_exceeds_length() {
    Playground::setup("str_test_11", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                     [package]
                     name = "rsh-arepas"
                 "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                 open sample.toml
                 | str substring 50..999 package.name
                 | get package.name
             "#
        ));

        assert_eq!(actual.out, "");
    })
}

#[test]
fn substrings_the_input_and_treats_start_index_as_zero_if_blank_start_index_given() {
    Playground::setup("str_test_12", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                     [package]
                     name = "rsh-arepas"
                 "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                 open sample.toml
                 | str substring ..2 package.name
                 | get package.name
             "#
        ));

        assert_eq!(actual.out, "rsh");
    })
}

#[test]
fn substrings_the_input_and_treats_end_index_as_length_if_blank_end_index_given() {
    Playground::setup("str_test_13", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContent(
            "sample.toml",
            r#"
                     [package]
                     name = "rsh-arepas"
                 "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                 open sample.toml
                 | str substring 3.. package.name
                 | get package.name
             "#
        ));

        assert_eq!(actual.out, "arepas");
    })
}

#[test]
fn str_reverse() {
    let actual = rsh!(r#"
        echo "rsh" | str reverse
        "#);

    assert!(actual.out.contains("llehsun"));
}

#[test]
fn test_redirection_trim() {
    let actual = rsh!(r#"
        let x = (rsh --testbin cococo niceone); $x | str trim | str length
        "#);

    assert_eq!(actual.out, "7");
}
