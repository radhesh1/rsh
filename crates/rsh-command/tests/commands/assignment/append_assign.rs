use rsh_test_support::rsh;

#[test]
fn append_assign_int() {
    let actual = rsh!(r#"
            mut a = [1 2];
            $a ++= [3 4];
            $a
        "#);

    let expected = rsh!(r#"
            [1 2 3 4]
        "#);

    print!("{}", actual.out);
    print!("{}", expected.out);
    assert_eq!(actual.out, expected.out);
}

#[test]
fn append_assign_string() {
    let actual = rsh!(r#"
            mut a = [a b];
            $a ++= [c d];
            $a
        "#);

    let expected = rsh!(r#"
            [a b c d]
        "#);

    print!("{}", actual.out);
    print!("{}", expected.out);
    assert_eq!(actual.out, expected.out);
}

#[test]
fn append_assign_any() {
    let actual = rsh!(r#"
            mut a = [1 2 a];
            $a ++= [b 3];
            $a
        "#);

    let expected = rsh!(r#"
            [1 2 a b 3]
        "#);

    print!("{}", actual.out);
    print!("{}", expected.out);
    assert_eq!(actual.out, expected.out);
}

#[test]
fn append_assign_both_empty() {
    let actual = rsh!(r#"
            mut a = [];
            $a ++= [];
            $a
        "#);

    let expected = rsh!(r#"
            []
        "#);

    print!("{}", actual.out);
    print!("{}", expected.out);
    assert_eq!(actual.out, expected.out);
}

#[test]
fn append_assign_type_mismatch() {
    let actual = rsh!(r#"
            mut a = [1 2];
            $a ++= [a];
            $a | to json -r;
        "#);

    assert_eq!(actual.out, r#"[1,2,"a"]"#);
}
