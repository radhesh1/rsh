use rsh_test_support::{rsh, rsh_repl_code};

#[test]
fn config_is_mutable() {
    let actual = rsh!(rsh_repl_code(&[
        r"$env.config = { ls: { clickable_links: true } }",
        "$env.config.ls.clickable_links = false;",
        "$env.config.ls.clickable_links"
    ]));

    assert_eq!(actual.out, "false");
}

#[test]
fn config_preserved_after_do() {
    let actual = rsh!(rsh_repl_code(&[
        r"$env.config = { ls: { clickable_links: true } }",
        "do -i { $env.config.ls.clickable_links = false }",
        "$env.config.ls.clickable_links"
    ]));

    assert_eq!(actual.out, "true");
}

#[test]
fn config_affected_when_mutated() {
    let actual = rsh!(rsh_repl_code(&[
        r#"$env.config = { filesize: { metric: false, format:"auto" } }"#,
        r#"$env.config = { filesize: { metric: true, format:"auto" } }"#,
        "20mib | into string"
    ]));

    assert_eq!(actual.out, "21.0 MB");
}

#[test]
fn config_affected_when_deep_mutated() {
    let actual = rsh!(cwd: "crates/rsh-utils/src/sample_config", rsh_repl_code(&[
        r#"source default_config.rsh"#,
        r#"$env.config.filesize.metric = true"#,
        r#"20mib | into string"#]));

    assert_eq!(actual.out, "21.0 MB");
}

#[test]
fn config_add_unsupported_key() {
    let actual = rsh!(cwd: "crates/rsh-utils/src/sample_config", rsh_repl_code(&[
        r#"source default_config.rsh"#,
        r#"$env.config.foo = 2"#,
        r#";"#]));

    assert!(actual
        .err
        .contains("$env.config.foo is an unknown config setting"));
}

#[test]
fn config_add_unsupported_type() {
    let actual = rsh!(cwd: "crates/rsh-utils/src/sample_config", rsh_repl_code(&[r#"source default_config.rsh"#,
        r#"$env.config.ls = '' "#,
        r#";"#]));

    assert!(actual.err.contains("should be a record"));
}

#[test]
fn config_add_unsupported_value() {
    let actual = rsh!(cwd: "crates/rsh-utils/src/sample_config", rsh_repl_code(&[r#"source default_config.rsh"#,
        r#"$env.config.history.file_format = ''"#,
        r#";"#]));

    assert!(actual
        .err
        .contains("unrecognized $env.config.history.file_format option ''"));
    assert!(actual
        .err
        .contains("expected either 'sqlite' or 'plaintext'"));
}

#[test]
#[ignore = "Figure out how to make test_bins::rsh_repl() continue execution after shell errors"]
fn config_unsupported_key_reverted() {
    let actual = rsh!(cwd: "crates/rsh-utils/src/sample_config", rsh_repl_code(&[r#"source default_config.rsh"#,
        r#"$env.config.foo = 1"#,
        r#"'foo' in $env.config"#]));

    assert_eq!(actual.out, "false");
}

#[test]
#[ignore = "Figure out how to make test_bins::rsh_repl() continue execution after shell errors"]
fn config_unsupported_type_reverted() {
    let actual = rsh!(cwd: "crates/rsh-utils/src/sample_config", rsh_repl_code(&[r#" source default_config.rsh"#,
        r#"$env.config.ls = ''"#,
        r#"$env.config.ls | describe"#]));

    assert_eq!(actual.out, "record");
}

#[test]
#[ignore = "Figure out how to make test_bins::rsh_repl() continue execution after errors"]
fn config_unsupported_value_reverted() {
    let actual = rsh!(cwd: "crates/rsh-utils/src/sample_config", rsh_repl_code(&[r#" source default_config.rsh"#,
        r#"$env.config.history.file_format = 'plaintext'"#,
        r#"$env.config.history.file_format = ''"#,
        r#"$env.config.history.file_format | to json"#]));

    assert_eq!(actual.out, "\"plaintext\"");
}
