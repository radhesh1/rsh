use rsh_test_support::{rsh, pipeline};

#[test]
fn formatter_not_valid() {
    let actual = rsh!(r#"
        date now | format date '%N'
        "#);

    assert!(actual.err.contains("invalid format"));
}

#[test]
fn fails_without_input() {
    let actual = rsh!(r#"
        format date "%c"
        "#);

    assert!(actual.err.contains("Pipeline empty"));
}

#[test]
fn locale_format_respect_different_locale() {
    let actual = rsh!(
        locale: "en_US",
        pipeline(
            r#"
            "2021-10-22 20:00:12 +01:00" | format date "%c"
            "#
        )
    );
    assert!(actual.out.contains("Fri 22 Oct 2021 08:00:12 PM +01:00"));

    let actual = rsh!(
        locale: "en_GB",
        pipeline(
            r#"
            "2021-10-22 20:00:12 +01:00" | format date "%c"
            "#
        )
    );
    assert!(actual.out.contains("Fri 22 Oct 2021 20:00:12 +01:00"));

    let actual = rsh!(
        locale: "de_DE",
        pipeline(
            r#"
            "2021-10-22 20:00:12 +01:00" | format date "%c"
            "#
        )
    );
    assert!(actual.out.contains("Fr 22 Okt 2021 20:00:12 +01:00"));

    let actual = rsh!(
        locale: "zh_CN",
        pipeline(
            r#"
            "2021-10-22 20:00:12 +01:00" | format date "%c"
            "#
        )
    );
    assert!(actual.out.contains("2021年10月22日 星期五 20时00分12秒"));

    let actual = rsh!(
        locale: "ja_JP",
        pipeline(
            r#"
            "2021-10-22 20:00:12 +01:00" | format date "%c"
            "#
        )
    );
    assert!(actual.out.contains("2021年10月22日 20時00分12秒"));

    let actual = rsh!(
        locale: "fr_FR",
        pipeline(
            r#"
            "2021-10-22 20:00:12 +01:00" | format date "%c"
            "#
        )
    );
    assert!(actual.out.contains("ven. 22 oct. 2021 20:00:12 +01:00"));
}
