use rsh_test_support::fs::Stub::{EmptyFile, FileWithContentToBeTrimmed};
use rsh_test_support::playground::Playground;
use rsh_test_support::{rsh, pipeline};

#[test]
fn creates_the_resulting_string_from_the_given_fields() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
        open cargo_sample.toml
            | get package
            | format "{name} has license {license}"
        "#
    ));

    assert_eq!(actual.out, "rsh has license ISC");
}

#[test]
fn format_input_record_output_string() {
    let actual = rsh!(r#"{name: Downloads} | format "{name}""#);

    assert_eq!(actual.out, "Downloads");
}

#[test]
fn given_fields_can_be_column_paths() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
        open cargo_sample.toml
            | format "{package.name} is {package.description}"
        "#
    ));

    assert_eq!(actual.out, "rsh is a new type of shell");
}

#[test]
fn can_use_variables() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
        open cargo_sample.toml
            | format "{$it.package.name} is {$it.package.description}"
        "#
    ));

    assert_eq!(actual.out, "rsh is a new type of shell");
}

#[test]
fn error_unmatched_brace() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
        open cargo_sample.toml
            | format "{$it.package.name"
        "#
    ));

    assert!(actual.err.contains("unmatched curly brace"));
}

#[test]
fn format_filesize_works() {
    Playground::setup("format_filesize_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![
            EmptyFile("yehuda.txt"),
            EmptyFile("jttxt"),
            EmptyFile("andres.txt"),
        ]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                ls
                | format filesize KB size
                | get size
                | first
            "
        ));

        assert_eq!(actual.out, "0.0 KB");
    })
}

#[test]
fn format_filesize_works_with_nonempty_files() {
    Playground::setup(
        "format_filesize_works_with_nonempty_files",
        |dirs, sandbox| {
            sandbox.with_files(vec![FileWithContentToBeTrimmed(
                "sample.toml",
                r#"
                    [dependency]
                    name = "rsh"
                "#,
            )]);

            let actual = rsh!(
                cwd: dirs.test(),
                "ls sample.toml | format filesize B size | get size | first"
            );

            #[cfg(not(windows))]
            assert_eq!(actual.out, "25");

            #[cfg(windows)]
            assert_eq!(actual.out, "27");
        },
    )
}
