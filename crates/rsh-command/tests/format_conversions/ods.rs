use rsh_test_support::{rsh, pipeline};

#[test]
fn from_ods_file_to_table() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            open sample_data.ods
            | get SalesOrders
            | get 4
            | get column2
        "#
    ));

    assert_eq!(actual.out, "Gill");
}

#[test]
fn from_ods_file_to_table_select_sheet() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            open sample_data.ods --raw
            | from ods --sheets ["SalesOrders"]
            | columns
            | get 0
        "#
    ));

    assert_eq!(actual.out, "SalesOrders");
}
