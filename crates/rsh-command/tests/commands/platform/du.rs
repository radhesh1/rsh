use rsh_test_support::{rsh, pipeline};

#[test]
fn test_du_flag_min_size() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            du -m -1
        "#
    ));
    assert!(actual
        .err
        .contains("Negative value passed when positive one is required"));

    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            du -m 1
        "#
    ));
    assert!(actual.err.is_empty());
}

#[test]
fn test_du_flag_max_depth() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            du -d -2
        "#
    ));
    assert!(actual
        .err
        .contains("Negative value passed when positive one is required"));

    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            du -d 2
        "#
    ));
    assert!(actual.err.is_empty());
}
