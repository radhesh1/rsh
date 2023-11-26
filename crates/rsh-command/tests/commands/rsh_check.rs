use rsh_test_support::fs::Stub::FileWithContentToBeTrimmed;
use rsh_test_support::playground::Playground;
use rsh_test_support::{rsh, pipeline};

#[test]
fn parse_script_success() {
    Playground::setup("rsh_check_test_1", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "script.rsh",
            r#"
                greet "world"

                def greet [name] {
                  echo "hello" $name
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check script.rsh
            "
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_script_with_wrong_type() {
    Playground::setup("rsh_check_test_2", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "script.rsh",
            r#"
                greet "world"

                def greet [name] {
                  echo "hello" $name
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check --debug --as-module script.rsh
            "
        ));

        assert!(actual.err.contains("Failed to parse content"));
    })
}
#[test]
fn parse_script_failure() {
    Playground::setup("rsh_check_test_3", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "script.rsh",
            r#"
                greet "world"

                def greet [name {
                  echo "hello" $name
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check --debug script.rsh
            "
        ));

        assert!(actual.err.contains("Unexpected end of code"));
    })
}

#[test]
fn parse_module_success() {
    Playground::setup("rsh_check_test_4", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.rsh",
            r#"
                # foo.rsh

                export def hello [name: string] {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check --as-module foo.rsh
            "
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_module_with_wrong_type() {
    Playground::setup("rsh_check_test_5", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.rsh",
            r#"
                # foo.rsh

                export def hello [name: string {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check --debug foo.rsh
            "
        ));

        assert!(actual.err.contains("Failed to parse content"));
    })
}
#[test]
fn parse_module_failure() {
    Playground::setup("rsh_check_test_6", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.rsh",
            r#"
                # foo.rsh

                export def hello [name: string {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check --debug --as-module foo.rsh
            "
        ));

        assert!(actual.err.contains("Unexpected end of code"));
    })
}

#[test]
fn file_not_exist() {
    Playground::setup("rsh_check_test_7", |dirs, _sandbox| {
        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check --as-module foo.rsh
            "
        ));

        assert!(actual.err.contains("file not found"));
    })
}

#[test]
fn parse_unsupported_file() {
    Playground::setup("rsh_check_test_8", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.txt",
            r#"
                # foo.rsh

                export def hello [name: string {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check --as-module foo.txt
            "
        ));

        assert!(actual
            .err
            .contains("File extension must be the type of .rsh"));
    })
}
#[test]
fn parse_dir_failure() {
    Playground::setup("rsh_check_test_9", |dirs, _sandbox| {
        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check --as-module ~
            "
        ));

        assert!(actual
            .err
            .contains("File extension must be the type of .rsh"));
    })
}

#[test]
fn parse_module_success_2() {
    Playground::setup("rsh_check_test_10", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.rsh",
            r#"
                # foo.rsh

                export-env { $env.MYNAME = "Arthur, King of the Britons" }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check --as-module foo.rsh
            "
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_script_success_with_raw_stream() {
    Playground::setup("rsh_check_test_11", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "script.rsh",
            r#"
                greet "world"

                def greet [name] {
                  echo "hello" $name
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open script.rsh | rsh-check
            "
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_module_success_with_raw_stream() {
    Playground::setup("rsh_check_test_12", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.rsh",
            r#"
                # foo.rsh

                export def hello [name: string] {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open foo.rsh | rsh-check --as-module
            "
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_string_as_script_success() {
    Playground::setup("rsh_check_test_13", |dirs, _sandbox| {
        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                echo $'two(char nl)lines' | rsh-check
            "#
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_string_as_script() {
    Playground::setup("rsh_check_test_14", |dirs, _sandbox| {
        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            r#"
                echo $'two(char nl)lines' | rsh-check --debug --as-module
            "#
        ));

        println!("the output is {}", actual.err);
        assert!(actual.err.contains("Failed to parse content"));
    })
}

#[test]
fn parse_module_success_with_internal_stream() {
    Playground::setup("rsh_check_test_15", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "foo.rsh",
            r#"
                # foo.rsh

                export def hello [name: string] {
                    $"hello ($name)!"
                }

                export def hi [where: string] {
                    $"hi ($where)!"
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open foo.rsh | lines | rsh-check --as-module
            "
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_script_success_with_complex_internal_stream() {
    Playground::setup("rsh_check_test_16", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.rsh",
            r#"
                #grep for rsh
                def grep-rsh [
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-rsh search file.txt
                  #ls **/* | some_filter | grep-rsh search
                  #open file.txt | grep-rsh search
                ] {
                  if ($entrada | is-empty) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f|
                      $f.match
                      | rsh-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open grep.rsh | lines | rsh-check
            "
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_script_failure_with_complex_internal_stream() {
    Playground::setup("rsh_check_test_17", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.rsh",
            r#"
                #grep for rsh
                def grep-rsh [
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-rsh search file.txt
                  #ls **/* | some_filter | grep-rsh search
                  #open file.txt | grep-rsh search
                ]
                  if ($entrada | is-empty) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f|
                      $f.match
                      | rsh-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open grep.rsh | lines | rsh-check
            "
        ));

        assert_eq!(actual.out, "false".to_string());
    })
}

#[test]
fn parse_script_success_with_complex_external_stream() {
    Playground::setup("rsh_check_test_18", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.rsh",
            r#"
                #grep for rsh
                def grep-rsh [
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-rsh search file.txt
                  #ls **/* | some_filter | grep-rsh search
                  #open file.txt | grep-rsh search
                ] {
                  if ($entrada | is-empty) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f|
                      $f.match
                      | rsh-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open grep.rsh | rsh-check
            "
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_module_success_with_complex_external_stream() {
    Playground::setup("rsh_check_test_19", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.rsh",
            r#"
                #grep for rsh
                def grep-rsh [
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-rsh search file.txt
                  #ls **/* | some_filter | grep-rsh search
                  #open file.txt | grep-rsh search
                ] {
                  if ($entrada | is-empty) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f|
                      $f.match
                      | rsh-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open grep.rsh | rsh-check --debug --as-module
            "
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_with_flag_all_success_for_complex_external_stream() {
    Playground::setup("rsh_check_test_20", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.rsh",
            r#"
                #grep for rsh
                def grep-rsh [
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-rsh search file.txt
                  #ls **/* | some_filter | grep-rsh search
                  #open file.txt | grep-rsh search
                ] {
                  if ($entrada | is-empty) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f|
                      $f.match
                      | rsh-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open grep.rsh | rsh-check --all --debug
            "
        ));

        assert!(actual.err.is_empty());
    })
}

#[test]
fn parse_with_flag_all_failure_for_complex_external_stream() {
    Playground::setup("rsh_check_test_21", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.rsh",
            r#"
                #grep for rsh
                def grep-rsh
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-rsh search file.txt
                  #ls **/* | some_filter | grep-rsh search
                  #open file.txt | grep-rsh search
                ] {
                  if ($entrada | is-empty) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f|
                      $f.match
                      | rsh-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open grep.rsh | rsh-check --all --debug
            "
        ));

        assert!(actual.err.contains("syntax error"));
    })
}

#[test]
fn parse_with_flag_all_failure_for_complex_list_stream() {
    Playground::setup("rsh_check_test_22", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "grep.rsh",
            r#"
                #grep for rsh
                def grep-rsh
                  search   #search term
                  entrada?  #file or pipe
                  #
                  #Examples
                  #grep-rsh search file.txt
                  #ls **/* | some_filter | grep-rsh search
                  #open file.txt | grep-rsh search
                ] {
                  if ($entrada | is-empty) {
                    if ($in | column? name) {
                      grep -ihHn $search ($in | get name)
                    } else {
                      ($in | into string) | grep -ihHn $search
                    }
                  } else {
                      grep -ihHn $search $entrada
                  }
                  | lines
                  | parse "{file}:{line}:{match}"
                  | str trim
                  | update match {|f|
                      $f.match
                      | rsh-highlight
                    }
                  | rename "source file" "line number"
                }

            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                open grep.rsh | lines | rsh-check --all --debug
            "
        ));

        assert!(actual.err.contains("syntax error"));
    })
}

#[test]
fn parse_failure_due_conflicted_flags() {
    Playground::setup("rsh_check_test_23", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "script.rsh",
            r#"
                greet "world"

                def greet [name] {
                  echo "hello" $name
                }
            "#,
        )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check -a --as-module script.rsh
            "
        ));

        assert!(actual
            .err
            .contains("You cannot have both `--all` and `--as-module` on the same command line"));
    })
}

#[test]
fn parse_script_with_nested_scripts_success() {
    Playground::setup("rsh_check_test_24", |dirs, sandbox| {
        sandbox
            .mkdir("lol")
            .with_files(vec![FileWithContentToBeTrimmed(
                "lol/lol.rsh",
                r#"
                    source-env ../foo.rsh
                    use lol_shell.rsh
                    overlay use ../lol/lol_shell.rsh
                "#,
            )])
            .with_files(vec![FileWithContentToBeTrimmed(
                "lol/lol_shell.rsh",
                r#"
                    export def ls [] { "lol" }
                "#,
            )])
            .with_files(vec![FileWithContentToBeTrimmed(
                "foo.rsh",
                r#"
                    $env.FOO = 'foo'
                "#,
            )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                rsh-check lol/lol.rsh
            "
        ));

        assert_eq!(actual.out, "true");
    })
}

#[test]
fn rsh_check_respects_file_pwd() {
    Playground::setup("rsh_check_test_25", |dirs, sandbox| {
        sandbox
            .mkdir("lol")
            .with_files(vec![FileWithContentToBeTrimmed(
                "lol/lol.rsh",
                r#"
                    $env.RETURN = (rsh-check ../foo.rsh)
                "#,
            )])
            .with_files(vec![FileWithContentToBeTrimmed(
                "foo.rsh",
                r#"
                    echo 'foo'
                "#,
            )]);

        let actual = rsh!(
            cwd: dirs.test(), pipeline(
            "
                source-env lol/lol.rsh;
                $env.RETURN
            "
        ));

        assert_eq!(actual.out, "true");
    })
}
