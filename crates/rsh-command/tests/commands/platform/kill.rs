use rsh_test_support::rsh;

#[test]
fn test_kill_invalid_pid() {
    let pid = i32::MAX;
    let actual = rsh!(format!("kill {pid}"));

    assert!(actual.err.contains("process didn't terminate successfully"));
}
