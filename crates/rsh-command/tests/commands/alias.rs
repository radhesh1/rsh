use rsh_test_support::fs::Stub::FileWithContentToBeTrimmed;
use rsh_test_support::playground::Playground;
use rsh_test_support::{rsh, pipeline};

#[ignore = "TODO?: Aliasing parser keywords does not work anymore"]
#[test]
fn alias_simple() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            alias bar = use sample_def.rsh greet;
            bar;
            greet
        "#
    ));

    assert_eq!(actual.out, "hello");
}

#[ignore = "TODO?: Aliasing parser keywords does not work anymore"]
#[test]
fn alias_hiding_1() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            overlay use ./activate-foo.rsh;
            scope aliases | find deactivate-foo | length
        "#
    ));

    assert_eq!(actual.out, "1");
}

#[ignore = "TODO?: Aliasing parser keywords does not work anymore"]
#[test]
fn alias_hiding_2() {
    let actual = rsh!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            overlay use ./activate-foo.rsh;
            deactivate-foo;
            scope aliases | find deactivate-foo | length
        "#
    ));

    assert_eq!(actual.out, "0");
}

#[test]
fn alias_fails_with_invalid_name() {
    let err_msg = "name can't be a number, a filesize, or contain a hash # or caret ^";
    let actual = rsh!(r#" alias 1234 = echo "test" "#);

    assert!(actual.err.contains(err_msg));

    let actual = rsh!(r#" alias 5gib = echo "test" "#);
    assert!(actual.err.contains(err_msg));

    let actual = rsh!(r#" alias "te#t" = echo "test" "#);
    assert!(actual.err.contains(err_msg));

    let actual = rsh!(r#" alias ^foo = echo "bar" "#);
    assert!(actual.err.contains(err_msg));
}

#[test]
fn cant_alias_keyword() {
    let actual = rsh!(r#" alias ou = let "#);
    assert!(actual.err.contains("cant_alias_keyword"));
}

#[test]
fn alias_wont_recurse() {
    let actual = rsh!(pipeline(
        "
            module myspamsymbol {
                export def myfoosymbol [prefix: string, msg: string] {
                    $prefix + $msg
                }
            };
            use myspamsymbol myfoosymbol;
            alias myfoosymbol = myfoosymbol 'hello';
            myfoosymbol ' world'
        "
    ));

    assert_eq!(actual.out, "hello world");
    assert!(actual.err.is_empty());
}

// Issue https://github.com/radhesh1/rsh/issues/8246
#[test]
fn alias_wont_recurse2() {
    Playground::setup("alias_wont_recurse2", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "spam.rsh",
            r#"
                def eggs [] { spam 'eggs' }
                alias spam = spam 'spam'
            "#,
        )]);
        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                def spam [what: string] { 'spam ' + $what };
                source spam.rsh;
                spam
            "#
        ));

        assert_eq!(actual.out, "spam spam");
        assert!(actual.err.is_empty());
    })
}

#[test]
fn alias_invalid_expression() {
    let actual = rsh!(r#" alias spam = 'foo' "#);
    assert!(actual.err.contains("cant_alias_expression"));

    let actual = rsh!(r#" alias spam = ([1 2 3] | length) "#);
    assert!(actual.err.contains("cant_alias_expression"));

    let actual = rsh!(r#" alias spam = 0..12 "#);
    assert!(actual.err.contains("cant_alias_expression"));
}

#[test]
fn alias_if() {
    let actual = rsh!(r#" alias spam = if true { 'spam' } else { 'eggs' }; spam "#);
    assert_eq!(actual.out, "spam");
}

#[test]
fn alias_match() {
    let actual = rsh!(r#" alias spam = match 3 { 1..10 => 'yes!' }; spam "#);
    assert_eq!(actual.out, "yes!");
}

// Issue https://github.com/radhesh1/rsh/issues/8103
#[test]
fn alias_multiword_name() {
    let actual = rsh!(r#"alias `foo bar` = echo 'test'; foo bar"#);
    assert_eq!(actual.out, "test");

    let actual = rsh!(r#"alias 'foo bar' = echo 'test'; foo bar"#);
    assert_eq!(actual.out, "test");

    let actual = rsh!(r#"alias "foo bar" = echo 'test'; foo bar"#);
    assert_eq!(actual.out, "test");
}

#[test]
fn alias_ordering() {
    let actual = rsh!(r#"alias bar = echo; def echo [] { 'dummy echo' }; bar 'foo'"#);
    assert_eq!(actual.out, "foo");
}
