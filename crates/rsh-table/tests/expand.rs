mod common;

use common::{create_row, create_table};

use rsh_table::{rshTableConfig, TableTheme as theme};

#[test]
fn test_expand() {
    let table = create_table(
        vec![create_row(4); 3],
        RshTableConfig {
            theme: theme::rounded(),
            with_header: true,
            expand: true,
            ..Default::default()
        },
        50,
    );

    assert_eq!(
        table.unwrap(),
        "╭────────────┬───────────┬───────────┬───────────╮\n\
         │     0      │     1     │     2     │     3     │\n\
         ├────────────┼───────────┼───────────┼───────────┤\n\
         │ 0          │ 1         │ 2         │ 3         │\n\
         │ 0          │ 1         │ 2         │ 3         │\n\
         ╰────────────┴───────────┴───────────┴───────────╯"
    );
}
