use rsh_test_support::{rsh, pipeline};

#[test]
fn generate_no_next_break() {
    let actual = rsh!(
        "generate 1 {|x| if $x == 3 { {out: $x}} else { {out: $x, next: ($x + 1)} }} | to nuon"
    );

    assert_eq!(actual.out, "[1, 2, 3]");
}

#[test]
fn generate_rshll_break() {
    let actual = rsh!("generate 1 {|x| if $x <= 3 { {out: $x, next: ($x + 1)} }} | to nuon");

    assert_eq!(actual.out, "[1, 2, 3]");
}

#[test]
fn generate_allows_empty_output() {
    let actual = rsh!(pipeline(
        r#"
        generate 0 {|x|
          if $x == 1 {
            {next: ($x + 1)}
          } else if $x < 3 {
            {out: $x, next: ($x + 1)}
          }
        } | to nuon
      "#
    ));

    assert_eq!(actual.out, "[0, 2]");
}

#[test]
fn generate_allows_no_output() {
    let actual = rsh!(pipeline(
        r#"
        generate 0 {|x|
          if $x < 3 {
            {next: ($x + 1)}
          }
        } | to nuon
      "#
    ));

    assert_eq!(actual.out, "[]");
}

#[test]
fn generate_allows_null_state() {
    let actual = rsh!(pipeline(
        r#"
        generate 0 {|x|
          if $x == null {
            {out: "done"}
          } else if $x < 1 {
            {out: "going", next: ($x + 1)}
          } else {
            {out: "stopping", next: null}
          }
        } | to nuon
      "#
    ));

    assert_eq!(actual.out, "[going, stopping, done]");
}

#[test]
fn generate_allows_null_output() {
    let actual = rsh!(pipeline(
        r#"
        generate 0 {|x|
          if $x == 3 {
            {out: "done"}
          } else {
            {out: null, next: ($x + 1)}
          }
        } | to nuon
      "#
    ));

    assert_eq!(actual.out, "[null, null, null, done]");
}

#[test]
fn generate_disallows_extra_keys() {
    let actual = rsh!("generate 0 {|x| {foo: bar, out: $x}}");
    assert!(actual.err.contains("Invalid block return"));
}

#[test]
fn generate_disallows_list() {
    let actual = rsh!("generate 0 {|x| [$x, ($x + 1)]}");
    assert!(actual.err.contains("Invalid block return"));
}

#[test]
fn generate_disallows_primitive() {
    let actual = rsh!("generate 0 {|x| 1}");
    assert!(actual.err.contains("Invalid block return"));
}
