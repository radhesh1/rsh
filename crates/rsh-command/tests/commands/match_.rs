use rsh_test_support::rsh;

#[test]
fn match_for_range() {
    let actual = rsh!(r#"match 3 { 1..10 => { print "success" } }"#);
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success");
}

#[test]
fn match_for_range_unmatched() {
    let actual = rsh!(r#"match 11 { 1..10 => { print "failure" }, _ => { print "success" }}"#);
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success");
}

#[test]
fn match_for_record() {
    let actual = rsh!("match {a: 11} { {a: $b} => { print $b }}");
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "11");
}

#[test]
fn match_for_record_shorthand() {
    let actual = rsh!("match {a: 12} { {$a} => { print $a }}");
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "12");
}

#[test]
fn match_list() {
    let actual = rsh!(
        r#"match [1, 2] { [$a] => { print $"single: ($a)" }, [$b, $c] => {print $"double: ($b) ($c)"}}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "double: 1 2");
}

#[test]
fn match_list_rest_ignore() {
    let actual = rsh!(
        r#"match [1, 2] { [$a, ..] => { print $"single: ($a)" }, [$b, $c] => {print $"double: ($b) ($c)"}}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "single: 1");
}

#[test]
fn match_list_rest() {
    let actual = rsh!(
        r#"match [1, 2, 3] { [$a, ..$remainder] => { print $"single: ($a) ($remainder | math sum)" }, [$b, $c] => {print $"double: ($b) ($c)"}}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "single: 1 5");
}

#[test]
fn match_constant_1() {
    let actual = rsh!(
        r#"match 2 { 1 => { print "failure"}, 2 => { print "success" }, 3 => { print "failure" }}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success");
}

#[test]
fn match_constant_2() {
    let actual = rsh!(
        r#"match 2.3 { 1.4 => { print "failure"}, 2.3 => { print "success" }, 3 => { print "failure" }}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success");
}

#[test]
fn match_constant_3() {
    let actual = rsh!(
        r#"match true { false => { print "failure"}, true => { print "success" }, 3 => { print "failure" }}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success");
}

#[test]
fn match_constant_4() {
    let actual = rsh!(
        r#"match "def" { "abc" => { print "failure"}, "def" => { print "success" }, "ghi" => { print "failure" }}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success");
}

#[test]
fn match_constant_5() {
    let actual = rsh!(
        r#"match 2023-08-23 { 2010-01-01 => { print "failure"}, 2023-08-23 => { print "success" }, 2020-02-02 => { print "failure" }}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success");
}

#[test]
fn match_constant_6() {
    let actual = rsh!(
        r#"match 6sec { 2sec => { print "failure"}, 6sec => { print "success" }, 1min => { print "failure" }}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success");
}

#[test]
fn match_constant_7() {
    let actual = rsh!(
        r#"match 1kib { 1kb => { print "failure"}, 1kib => { print "success" }, 2kb => { print "failure" }}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success");
}

#[test]
fn match_null() {
    let actual = rsh!(r#"match null { null => { print "success"}, _ => { print "failure" }}"#);
    assert_eq!(actual.out, "success");
}

#[test]
fn match_or_pattern() {
    let actual = rsh!(
        r#"match {b: 7} { {a: $a} | {b: $b} => { print $"success: ($b)" }, _ => { print "failure" }}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success: 7");
}

#[test]
fn match_or_pattern_overlap_1() {
    let actual = rsh!(
        r#"match {a: 7} { {a: $b} | {b: $b} => { print $"success: ($b)" }, _ => { print "failure" }}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success: 7");
}

#[test]
fn match_or_pattern_overlap_2() {
    let actual = rsh!(
        r#"match {b: 7} { {a: $b} | {b: $b} => { print $"success: ($b)" }, _ => { print "failure" }}"#
    );
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "success: 7");
}

#[test]
fn match_doesnt_overwrite_variable() {
    let actual = rsh!("let b = 100; match 55 { $b => {} }; print $b");
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert_eq!(actual.out, "100");
}

#[test]
fn match_with_guard() {
    let actual = rsh!(
        cwd: ".",
        "match [1 2 3] { [$x, ..] if $x mod 2 == 0 => { $x }, $x => { 2 } }"
    );

    assert_eq!(actual.out, "2");
}

#[test]
fn match_with_guard_block_as_guard() {
    // this should work?
    let actual = rsh!(
        cwd: ".",
        "match 4 { $x if { $x + 20 > 25 } => { 'good num' }, _ => { 'terrible num' } }"
    );

    assert!(actual.err.contains("Match guard not bool"));
}

#[test]
fn match_with_guard_parens_expr_as_guard() {
    let actual = rsh!(
        cwd: ".",
        "match 4 { $x if ($x + 20 > 25) => { 'good num' }, _ => { 'terrible num' } }"
    );

    assert_eq!(actual.out, "terrible num");
}

#[test]
fn match_with_guard_not_bool() {
    let actual = rsh!(
        cwd: ".",
        "match 4 { $x if $x + 1 => { 'err!()' }, _ => { 'unreachable!()' } }"
    );

    assert!(actual.err.contains("Match guard not bool"));
}

#[test]
fn match_with_guard_no_expr_after_if() {
    let actual = rsh!(
        cwd: ".",
        "match 4 { $x if  => { 'err!()' }, _ => { 'unreachable!()' } }"
    );

    assert!(actual.err.contains("Match guard without an expression"));
}
