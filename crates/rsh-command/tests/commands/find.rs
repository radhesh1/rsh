use rsh_test_support::rsh;

#[test]
fn find_with_list_search_with_string() {
    let actual = rsh!("[moe larry curly] | find moe | get 0");

    assert_eq!(actual.out, "moe");
}

#[test]
fn find_with_list_search_with_char() {
    let actual = rsh!("[moe larry curly] | find l | to json -r");

    assert_eq!(actual.out, r#"["larry","curly"]"#);
}

#[test]
fn find_with_list_search_with_number() {
    let actual = rsh!("[1 2 3 4 5] | find 3 | get 0");

    assert_eq!(actual.out, "3");
}

#[test]
fn find_with_string_search_with_string() {
    let actual = rsh!("echo Cargo.toml | find toml");

    assert_eq!(actual.out, "Cargo.toml");
}

#[test]
fn find_with_string_search_with_string_not_found() {
    let actual = rsh!("[moe larry curly] | find shemp | is-empty");

    assert_eq!(actual.out, "true");
}

#[test]
fn find_with_filepath_search_with_string() {
    let actual =
        rsh!(r#"["amigos.txt","arepas.clu","los.txt","tres.txt"] | find arep | to json -r"#);

    assert_eq!(actual.out, r#"["arepas.clu"]"#);
}

#[test]
fn find_with_filepath_search_with_multiple_patterns() {
    let actual =
        rsh!(r#"["amigos.txt","arepas.clu","los.txt","tres.txt"] | find arep ami | to json -r"#);

    assert_eq!(actual.out, r#"["amigos.txt","arepas.clu"]"#);
}

#[test]
fn find_takes_into_account_linebreaks_in_string() {
    let actual = rsh!(r#""atest\nanothertest\nnohit\n" | find a | length"#);

    assert_eq!(actual.out, "2");
}

#[test]
fn find_with_regex_in_table_keeps_row_if_one_column_matches() {
    let actual = rsh!(
        "[[name nickname]; [Maurice moe] [Laurence larry]] | find --regex ce | get name | to json -r"
    );

    assert_eq!(actual.out, r#"["Maurice","Laurence"]"#);
}

#[test]
fn inverted_find_with_regex_in_table_keeps_row_if_none_of_the_columns_matches() {
    let actual = rsh!(
        "[[name nickname]; [Maurice moe] [Laurence larry]] | find --regex moe --invert | get name | to json -r"
    );

    assert_eq!(actual.out, r#"["Laurence"]"#);
}

#[test]
fn find_in_table_only_keep_rows_with_matches_on_selected_columns() {
    let actual = rsh!(
        "[[name nickname]; [Maurice moe] [Laurence larry]] | find r --columns [nickname] | get name | to json -r"
    );

    assert!(actual.out.contains("Laurence"));
    assert!(!actual.out.contains("Maurice"));
}

#[test]
fn inverted_find_in_table_keeps_row_if_none_of_the_selected_columns_matches() {
    let actual = rsh!(
        "[[name nickname]; [Maurice moe] [Laurence larry]] | find r --columns [nickname] --invert | get name | to json -r"
    );

    assert_eq!(actual.out, r#"["Maurice"]"#);
}

#[test]
fn find_in_table_keeps_row_with_single_matched_and_keeps_other_columns() {
    let actual = rsh!("[[name nickname Age]; [Maurice moe 23] [Laurence larry 67] [William will 18]] | find Maurice");

    println!("{:?}", actual.out);
    assert!(actual.out.contains("moe"));
    assert!(actual.out.contains("Maurice"));
    assert!(actual.out.contains("23"));
}

#[test]
fn find_in_table_keeps_row_with_multiple_matched_and_keeps_other_columns() {
    let actual = rsh!("[[name nickname Age]; [Maurice moe 23] [Laurence larry 67] [William will 18] [William bill 60]] | find moe William");

    println!("{:?}", actual.out);
    assert!(actual.out.contains("moe"));
    assert!(actual.out.contains("Maurice"));
    assert!(actual.out.contains("23"));
    assert!(actual.out.contains("William"));
    assert!(actual.out.contains("will"));
    assert!(actual.out.contains("18"));
    assert!(actual.out.contains("bill"));
    assert!(actual.out.contains("60"));
}
