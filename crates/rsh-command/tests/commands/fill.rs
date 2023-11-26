use rsh_test_support::{rsh, pipeline};

#[test]
fn string_fill_plain() {
    let actual = rsh!(r#""abc" | fill --alignment center --character "+" --width 5"#);

    assert_eq!(actual.out, "+abc+");
}

#[test]
fn string_fill_fancy() {
    let actual = rsh!(pipeline(
        r#"
            $"(ansi red)a(ansi green)\u{65}\u{308}(ansi cyan)c(ansi reset)" 
            | fill --alignment center --character "+" --width 5
            "#
    ));

    assert_eq!(
        actual.out,
        "+\u{1b}[31ma\u{1b}[32me\u{308}\u{1b}[36mc\u{1b}[0m+"
    );
}
