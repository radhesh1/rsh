mod table;
mod table_theme;
mod types;
mod unstructured_table;
mod util;

pub mod common;

pub use common::{StringResult, TableResult};
pub use rsh_color_config::TextStyle;
pub use table::{RshTable, RshTableCell, RshTableConfig};
pub use table_theme::TableTheme;
pub use types::{CollapsedTable, ExpandedTable, JustTable, TableOpts, TableOutput};
pub use unstructured_table::UnstructuredTable;
pub use util::*;
