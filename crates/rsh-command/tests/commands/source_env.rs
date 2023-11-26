use rsh_test_support::fs::AbsolutePath;
use rsh_test_support::fs::Stub::{FileWithContent, FileWithContentToBeTrimmed};
use rsh_test_support::rsh;
use rsh_test_support::pipeline;
use rsh_test_support::playground::Playground;

#[should_panic]
#[test]
fn sources_also_files_under_custom_lib_dirs_path() {
    Playground::setup("source_test_1", |dirs, rsh| {
        let file = AbsolutePath::new(dirs.test().join("config.toml"));
        let library_path = AbsolutePath::new(dirs.test().join("lib"));

        rsh.with_config(&file);
        rsh.with_files(vec![FileWithContent(
            "config.toml",
            &format!(
                r#"
                lib_dirs = ["{library_path}"]
                skip_welcome_message = true
            "#
            ),
        )]);

        rsh.within("lib").with_files(vec![FileWithContent(
            "my_library.rsh",
            r#"
                source-env my_library/main.rsh
            "#,
        )]);
        rsh.within("lib/my_library").with_files(vec![FileWithContent(
            "main.rsh",
            r#"
                $env.hello = "hello rsh"
            "#,
        )]);

        let actual = rsh!(
            cwd: ".", pipeline(
            "
                source-env my_library.rsh ;

                hello
            "
        ));

        assert_eq!(actual.out, "hello rsh");
    })
}

fn try_source_foo_with_double_quotes_in(testdir: &str, playdir: &str) {
    Playground::setup(playdir, |dirs, sandbox| {
        let testdir = String::from(testdir);
        let mut foo_file = testdir.clone();
        foo_file.push_str("/foo.rsh");

        sandbox.mkdir(&testdir);
        sandbox.with_files(vec![FileWithContent(&foo_file, "echo foo")]);

        let cmd = String::from("source-env ") + r#"""# + foo_file.as_str() + r#"""#;

        let actual = rsh!(cwd: dirs.test(), &cmd);

        assert_eq!(actual.out, "foo");
    });
}

fn try_source_foo_with_single_quotes_in(testdir: &str, playdir: &str) {
    Playground::setup(playdir, |dirs, sandbox| {
        let testdir = String::from(testdir);
        let mut foo_file = testdir.clone();
        foo_file.push_str("/foo.rsh");

        sandbox.mkdir(&testdir);
        sandbox.with_files(vec![FileWithContent(&foo_file, "echo foo")]);

        let cmd = String::from("source-env ") + r#"'"# + foo_file.as_str() + r#"'"#;

        let actual = rsh!(cwd: dirs.test(), &cmd);

        assert_eq!(actual.out, "foo");
    });
}

fn try_source_foo_without_quotes_in(testdir: &str, playdir: &str) {
    Playground::setup(playdir, |dirs, sandbox| {
        let testdir = String::from(testdir);
        let mut foo_file = testdir.clone();
        foo_file.push_str("/foo.rsh");

        sandbox.mkdir(&testdir);
        sandbox.with_files(vec![FileWithContent(&foo_file, "echo foo")]);

        let cmd = String::from("source-env ") + foo_file.as_str();

        let actual = rsh!(cwd: dirs.test(), &cmd);

        assert_eq!(actual.out, "foo");
    });
}

#[test]
fn sources_unicode_file_in_normal_dir() {
    try_source_foo_with_single_quotes_in("foo", "source_test_1");
    try_source_foo_with_double_quotes_in("foo", "source_test_2");
    try_source_foo_without_quotes_in("foo", "source_test_3");
}

#[test]
fn sources_unicode_file_in_unicode_dir_without_spaces_1() {
    try_source_foo_with_single_quotes_in("🚒", "source_test_4");
    try_source_foo_with_double_quotes_in("🚒", "source_test_5");
    try_source_foo_without_quotes_in("🚒", "source_test_6");
}

#[cfg(not(windows))] // ':' is not allowed in Windows paths
#[test]
fn sources_unicode_file_in_unicode_dir_without_spaces_2() {
    try_source_foo_with_single_quotes_in(":fire_engine:", "source_test_7");
    try_source_foo_with_double_quotes_in(":fire_engine:", "source_test_8");
    try_source_foo_without_quotes_in(":fire_engine:", "source_test_9");
}

#[test]
fn sources_unicode_file_in_unicode_dir_with_spaces_1() {
    // this one fails
    try_source_foo_with_single_quotes_in("e-$ èрт🚒♞中片-j", "source_test_8");
    // this one passes
    try_source_foo_with_double_quotes_in("e-$ èрт🚒♞中片-j", "source_test_9");
}

#[cfg(not(windows))] // ':' is not allowed in Windows paths
#[test]
fn sources_unicode_file_in_unicode_dir_with_spaces_2() {
    try_source_foo_with_single_quotes_in("e-$ èрт:fire_engine:♞中片-j", "source_test_10");
    try_source_foo_with_double_quotes_in("e-$ èрт:fire_engine:♞中片-j", "source_test_11");
}

#[ignore]
#[test]
fn sources_unicode_file_in_non_utf8_dir() {
    // How do I create non-UTF-8 path???
}

#[ignore]
#[test]
fn can_source_dynamic_path() {
    Playground::setup("can_source_dynamic_path", |dirs, sandbox| {
        let foo_file = "foo.rsh";

        sandbox.with_files(vec![FileWithContent(foo_file, "echo foo")]);

        let cmd = format!("let file = `{foo_file}`; source-env $file");
        let actual = rsh!(cwd: dirs.test(), &cmd);

        assert_eq!(actual.out, "foo");
    });
}

#[test]
fn source_env_eval_export_env() {
    Playground::setup("source_env_eval_export_env", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "spam.rsh",
            r#"
                export-env { $env.FOO = 'foo' }
            "#,
        )]);

        let inp = &[r#"source-env spam.rsh"#, r#"$env.FOO"#];

        let actual = rsh!(cwd: dirs.test(), &inp.join("; "));

        assert_eq!(actual.out, "foo");
    })
}

#[test]
fn source_env_eval_export_env_hide() {
    Playground::setup("source_env_eval_export_env", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "spam.rsh",
            r#"
                export-env { hide-env FOO }
            "#,
        )]);

        let inp = &[
            r#"$env.FOO = 'foo'"#,
            r#"source-env spam.rsh"#,
            r#"$env.FOO"#,
        ];

        let actual = rsh!(cwd: dirs.test(), &inp.join("; "));

        assert!(actual.err.contains("not_found"));
    })
}

#[test]
fn source_env_do_cd() {
    Playground::setup("source_env_do_cd", |dirs, sandbox| {
        sandbox
            .mkdir("test1/test2")
            .with_files(vec![FileWithContentToBeTrimmed(
                "test1/test2/spam.rsh",
                r#"
                    cd test1/test2
                "#,
            )]);

        let inp = &[
            r#"source-env test1/test2/spam.rsh"#,
            r#"$env.PWD | path basename"#,
        ];

        let actual = rsh!(cwd: dirs.test(), &inp.join("; "));

        assert_eq!(actual.out, "test2");
    })
}

#[test]
fn source_env_do_cd_file_relative() {
    Playground::setup("source_env_do_cd_file_relative", |dirs, sandbox| {
        sandbox
            .mkdir("test1/test2")
            .with_files(vec![FileWithContentToBeTrimmed(
                "test1/test2/spam.rsh",
                r#"
                    cd ($env.FILE_PWD | path join '..')
                "#,
            )]);

        let inp = &[
            r#"source-env test1/test2/spam.rsh"#,
            r#"$env.PWD | path basename"#,
        ];

        let actual = rsh!(cwd: dirs.test(), &inp.join("; "));

        assert_eq!(actual.out, "test1");
    })
}

#[test]
fn source_env_dont_cd_overlay() {
    Playground::setup("source_env_dont_cd_overlay", |dirs, sandbox| {
        sandbox
            .mkdir("test1/test2")
            .with_files(vec![FileWithContentToBeTrimmed(
                "test1/test2/spam.rsh",
                r#"
                    overlay new spam
                    cd test1/test2
                    overlay hide spam
                "#,
            )]);

        let inp = &[
            r#"source-env test1/test2/spam.rsh"#,
            r#"$env.PWD | path basename"#,
        ];

        let actual = rsh!(cwd: dirs.test(), &inp.join("; "));

        assert_eq!(actual.out, "source_env_dont_cd_overlay");
    })
}

#[test]
fn source_env_is_scoped() {
    Playground::setup("source_env_is_scoped", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "spam.rsh",
            r#"
                def no-name-similar-to-this [] { 'no-name-similar-to-this' }
                alias nor-similar-to-this = echo 'nor-similar-to-this'
            "#,
        )]);

        let inp = &[r#"source-env spam.rsh"#, r#"no-name-similar-to-this"#];

        let actual = rsh!(cwd: dirs.test(), &inp.join("; "));

        assert!(actual.err.contains("executable was not found"));

        let inp = &[r#"source-env spam.rsh"#, r#"nor-similar-to-this"#];

        let actual = rsh!(cwd: dirs.test(), &inp.join("; "));

        assert!(actual.err.contains("executable was not found"));
    })
}

#[test]
fn source_env_const_file() {
    Playground::setup("source_env_const_file", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "spam.rsh",
            r#"
                $env.FOO = 'foo'
            "#,
        )]);

        let inp = &[
            r#"const file = 'spam.rsh'"#,
            r#"source-env $file"#,
            r#"$env.FOO"#,
        ];

        let actual = rsh!(cwd: dirs.test(), &inp.join("; "));

        assert_eq!(actual.out, "foo");
    })
}

#[test]
fn source_respects_early_return() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        "
            source early_return.rsh
        "
    ));

    assert!(actual.err.is_empty());
}
