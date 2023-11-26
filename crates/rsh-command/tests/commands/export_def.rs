use rsh_test_support::rsh;

#[test]
fn export_subcommands_help() {
    let actual = rsh!("export def -h");

    assert!(actual
        .out
        .contains("Define a custom command and export it from a module"));
}
