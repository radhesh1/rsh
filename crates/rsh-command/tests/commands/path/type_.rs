use rsh_test_support::fs::Stub::EmptyFile;
use rsh_test_support::playground::Playground;
use rsh_test_support::{rsh, pipeline};

#[test]
fn returns_type_of_missing_file() {
    let actual = rsh!(
        cwd: "tests", pipeline(
        r#"
            echo "spam.txt"
            | path type
        "#
    ));

    assert_eq!(actual.out, "");
}

#[test]
fn returns_type_of_existing_file() {
    Playground::setup("path_expand_1", |dirs, sandbox| {
        sandbox
            .within("menu")
            .with_files(vec![EmptyFile("spam.txt")]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                echo "menu"
                | path type
            "#
        ));

        assert_eq!(actual.out, "dir");
    })
}

#[test]
fn returns_type_of_existing_directory() {
    Playground::setup("path_expand_1", |dirs, sandbox| {
        sandbox
            .within("menu")
            .with_files(vec![EmptyFile("spam.txt")]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                echo "menu/spam.txt"
                | path type
            "#
        ));

        assert_eq!(actual.out, "file");

        let actual = rsh!(pipeline(
            r#"
                echo "~"
                | path type
            "#
        ));

        assert_eq!(actual.out, "dir");
    })
}

#[test]
fn returns_type_of_existing_file_const() {
    Playground::setup("path_type_const", |dirs, sandbox| {
        sandbox
            .within("menu")
            .with_files(vec![EmptyFile("spam.txt")]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                const ty = ("menu" | path type);
                $ty
            "#
        ));

        assert_eq!(actual.out, "dir");
    })
}
