mod commands;
mod completions;
mod config_files;
mod eval_cmds;
mod eval_file;
mod menus;
mod rsh_highlight;
mod print;
mod prompt;
mod prompt_update;
mod reedline_config;
mod repl;
mod syntax_highlight;
mod util;
mod validation;

pub use commands::add_cli_context;
pub use completions::{FileCompletion, RshCompleter};
pub use config_files::eval_config_contents;
pub use eval_cmds::evaluate_commands;
pub use eval_file::evaluate_file;
pub use menus::{DescriptionMenu, RshHelpCompleter};
pub use rsh_cmd_base::util::get_init_cwd;
pub use rsh_highlight::RshHighlight;
pub use print::Print;
pub use prompt::RshPrompt;
pub use repl::evaluate_repl;
pub use syntax_highlight::RshHighlighter;
pub use util::{eval_source, gather_parent_env_vars};
pub use validation::RshValidator;

#[cfg(feature = "plugin")]
pub use config_files::add_plugin_file;
#[cfg(feature = "plugin")]
pub use config_files::read_plugin_file;
