use rsh_test_support::fs::Stub::FileWithContentToBeTrimmed;
use rsh_test_support::playground::Playground;
use rsh_test_support::{rsh, pipeline};

#[test]
fn to_column() {
    Playground::setup("split_column_test_1", |dirs, sandbox| {
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
                importer , shipper  , tariff_item  ,   name  ,  origin
            "#,
            ),
        ]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                open sample.txt
                | lines
                | str trim
                | split column ","
                | get column2
            "#
        ));

        assert!(actual.out.contains("shipper"));

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r"
                open sample2.txt
                | lines
                | str trim
                | split column --regex '\s*,\s*'
                | get column2
            "
        ));

        assert!(actual.out.contains("shipper"));
    })
}
