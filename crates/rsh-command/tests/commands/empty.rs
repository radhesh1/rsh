use rsh_test_support::{rsh, pipeline};

#[test]
fn reports_emptiness() {
    let actual = rsh!(pipeline(
        r#"
            [[] '' {} null]
            | all {||
              is-empty
            }
        "#
    ));

    assert_eq!(actual.out, "true");
}

#[test]
fn reports_nonemptiness() {
    let actual = rsh!(pipeline(
        r#"
            [[1] ' ' {a:1} 0]
            | any {||
              is-empty
            }
        "#
    ));

    assert_eq!(actual.out, "false");
}

#[test]
fn reports_emptiness_by_columns() {
    let actual = rsh!(pipeline(
        "
            [{a:1 b:null c:null} {a:2 b:null c:null}]
            | any {||
              is-empty b c
            }
        "
    ));

    assert_eq!(actual.out, "true");
}

#[test]
fn reports_nonemptiness_by_columns() {
    let actual = rsh!(pipeline(
        "
            [{a:1 b:null c:3} {a:null b:5 c:2}]
            | any {||
              is-empty a b
            }
        "
    ));

    assert_eq!(actual.out, "false");
}
