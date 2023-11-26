use rsh_test_support::fs::Stub::FileWithContentToBeTrimmed;
use rsh_test_support::playground::Playground;
use rsh_test_support::{rsh, pipeline};

#[test]
fn to_row() {
    Playground::setup("split_row_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![
            FileWithContentToBeTrimmed(
                "sample.txt",
                r#"
                importer,shipper,tariff_item,name,origin
            "#,
            ),
            FileWithContentToBeTrimmed(
                "sample2.txt",
                r#"
                importer      ,   shipper      ,  tariff_item,name      ,    origin
            "#,
            ),
        ]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                open sample.txt
                | lines
                | str trim
                | split row ","
                | length
            "#
        ));

        assert!(actual.out.contains('5'));

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r"
                open sample2.txt
                | lines
                | str trim
                | split row -r '\s*,\s*'
                | length
            "
        ));

        assert!(actual.out.contains('5'));

        let actual = rsh!(r#"
                def foo [a: list<string>] {
                    $a | describe
                }
                foo (["a b", "c d"] | split row " ")
            "#);

        assert!(actual.out.contains("list<string>"));
    })
}
