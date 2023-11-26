use rsh_test_support::rsh;

#[test]
fn loop_doesnt_auto_print_in_each_iteration() {
    let actual = rsh!("
        mut total = 0;
        loop {
            if $total == 3 {
                break;
            } else {
                $total += 1;
            }
            1
        }");
    // Make sure we don't see any of these values in the output
    // As we do not auto-print loops anymore
    assert!(!actual.out.contains('1'));
}

#[test]
fn loop_break_on_external_failed() {
    let actual = rsh!("
        mut total = 0;
        loop {
            if $total == 3 {
                break;
            } else {
                $total += 1;
            }
            print 1;
            rsh --testbin fail;
        }");
    // Note: rsh! macro auto replace "\n" and "\r\n" with ""
    // so our output will be `1`.
    assert_eq!(actual.out, "1");
}

#[test]
fn failed_loop_should_break_running() {
    let actual = rsh!("
        mut total = 0;
        loop {
            if $total == 3 {
                break;
            } else {
                $total += 1;
            }
            rsh --testbin fail;
        }
        print 3");
    assert!(!actual.out.contains('3'));
}
