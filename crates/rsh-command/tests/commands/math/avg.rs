use rsh_test_support::{rsh, pipeline};

#[test]
fn can_average_numbers() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
             open sgml_description.json
             | get glossary.GlossDiv.GlossList.GlossEntry.Sections
             | math avg
         "#
    ));

    assert_eq!(actual.out, "101.5")
}

#[test]
fn can_average_bytes() {
    let actual = rsh!("[100kb, 10b, 100mib] | math avg | to json -r");

    assert_eq!(actual.out, "34985870");
}
