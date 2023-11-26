use rsh_color_config::StyleComputer;
use rsh_protocol::{Record, Span, Value};
use rsh_table::{
    common::{rsh_value_to_string, rsh_value_to_string_clean},
    ExpandedTable, TableOpts,
};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::rsh_common::rshConfig;

pub fn try_build_table(
    ctrlc: Option<Arc<AtomicBool>>,
    config: &rshConfig,
    style_computer: &StyleComputer,
    value: Value,
) -> String {
    let span = value.span();
    match value {
        Value::List { vals, .. } => try_build_list(vals, ctrlc, config, span, style_computer),
        Value::Record { val, .. } => try_build_map(val, span, style_computer, ctrlc, config),
        val if matches!(val, Value::String { .. }) => {
            rsh_value_to_string_clean(&val, config, style_computer).0
        }
        val => rsh_value_to_string(&val, config, style_computer).0,
    }
}

fn try_build_map(
    record: Record,
    span: Span,
    style_computer: &StyleComputer,
    ctrlc: Option<Arc<AtomicBool>>,
    config: &rshConfig,
) -> String {
    let opts = TableOpts::new(
        config,
        style_computer,
        ctrlc,
        Span::unknown(),
        usize::MAX,
        (config.table_indent.left, config.table_indent.right),
        config.table_mode,
        0,
        false,
    );
    let result = ExpandedTable::new(None, false, String::new()).build_map(&record, opts);
    match result {
        Ok(Some(result)) => result,
        Ok(None) | Err(_) => {
            rsh_value_to_string(&Value::record(record, span), config, style_computer).0
        }
    }
}

fn try_build_list(
    vals: Vec<Value>,
    ctrlc: Option<Arc<AtomicBool>>,
    config: &rshConfig,
    span: Span,
    style_computer: &StyleComputer,
) -> String {
    let opts = TableOpts::new(
        config,
        style_computer,
        ctrlc,
        Span::unknown(),
        usize::MAX,
        (config.table_indent.left, config.table_indent.right),
        config.table_mode,
        0,
        false,
    );

    let result = ExpandedTable::new(None, false, String::new()).build_list(&vals, opts);
    match result {
        Ok(Some(out)) => out,
        Ok(None) | Err(_) => {
            // it means that the list is empty
            rsh_value_to_string(&Value::list(vals, span), config, style_computer).0
        }
    }
}
