use rsh_test_support::{rsh, pipeline};

#[test]
fn splits_empty_path() {
    let actual = rsh!(
        cwd: "tests", pipeline(
        r#"
            echo '' | path split | is-empty
        "#
    ));

    assert_eq!(actual.out, "true");
}

#[test]
fn splits_correctly_single_path() {
    let actual = rsh!(
        cwd: "tests", pipeline(
        r#"
            'home/viking/spam.txt'
            | path split
            | last
        "#
    ));

    assert_eq!(actual.out, "spam.txt");
}

#[test]
fn splits_correctly_single_path_const() {
    let actual = rsh!(r#"
        const result = ('home/viking/spam.txt' | path split);
        $result | last
    "#);

    assert_eq!(actual.out, "spam.txt");
}
