use rsh_test_support::rsh;

#[test]
fn into_datetime_table_column() {
    let actual = rsh!(r#"[[date]; ["2022-01-01"] ["2023-01-01"]] | into datetime date"#);

    assert!(actual.out.contains(" ago"));
}
