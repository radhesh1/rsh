use rsh_test_support::{rsh, pipeline};

#[test]
fn md_empty() {
    let actual = rsh!(r#"
            echo [[]; []] | from json | to md
        "#);

    assert_eq!(actual.out, "");
}

#[test]
fn md_empty_pretty() {
    let actual = rsh!(r#"
            echo "{}" | from json | to md -p
        "#);

    assert_eq!(actual.out, "");
}

#[test]
fn md_simple() {
    let actual = rsh!(r#"
            echo 3 | to md
        "#);

    assert_eq!(actual.out, "3");
}

#[test]
fn md_simple_pretty() {
    let actual = rsh!(r#"
            echo 3 | to md -p
        "#);

    assert_eq!(actual.out, "3");
}

#[test]
fn md_table() {
    let actual = rsh!(r#"
            echo [[name]; [jason]] | to md
        "#);

    assert_eq!(actual.out, "|name||-||jason|");
}

#[test]
fn md_table_pretty() {
    let actual = rsh!(r#"
            echo [[name]; [joseph]] | to md -p
        "#);

    assert_eq!(actual.out, "| name   || ------ || joseph |");
}

#[test]
fn md_combined() {
    let actual = rsh!(pipeline(
        r#"
            def title [] {
                echo [[H1]; ["rsh top meals"]]
            };

            def meals [] {
                echo [[dish]; [Arepa] [Taco] [Pizza]]
            };

            title
            | append (meals)
            | to md --per-element --pretty
        "#
    ));

    assert_eq!(
        actual.out,
        "# rsh top meals| dish  || ----- || Arepa || Taco  || Pizza |"
    );
}
