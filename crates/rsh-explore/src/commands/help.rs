use std::collections::HashMap;
use std::io::{self, Result};

use crossterm::event::KeyEvent;
use rsh_protocol::{
    engine::{EngineState, Stack},
    record, Record, Value,
};
use ratatui::layout::Rect;

use crate::{
    rsh_common::{collect_input, rshSpan},
    pager::{Frame, Transition, ViewInfo},
    views::{Layout, Preview, RecordView, View, ViewConfig},
};

use super::{HelpExample, HelpManual, ViewCommand};

#[derive(Debug, Default, Clone)]
pub struct HelpCmd {
    input_command: String,
    supported_commands: Vec<HelpManual>,
    aliases: HashMap<String, Vec<String>>,
}

impl HelpCmd {
    pub const NAME: &'static str = "help";

    const HELP_MESSAGE: &'static str = r#"                        Explore - main help file

              Move around:  Use the cursor keys.
               Close help:  Press "<Esc>".
             Exit Explore:  Type ":q" then then <Enter> (or press Ctrl+D).
 Open an interactive REPL:  Type ":try" then enter
    List all sub-commands:  Type ":help :" then <Enter>

------------------------------------------------------------------------------------

# Regular expressions

Most commands support regular expressions.

You can type "/" and type a pattern you want to search on.
Then hit <Enter> and you will see the search results.

To go to the next hit use "<n>" key.

You also can do a reverse search by using "?" instead of "/".
"#;

    pub fn new(commands: Vec<HelpManual>, aliases: &[(&str, &str)]) -> Self {
        let aliases = collect_aliases(aliases);

        Self {
            input_command: String::new(),
            supported_commands: commands,
            aliases,
        }
    }
}

fn collect_aliases(aliases: &[(&str, &str)]) -> HashMap<String, Vec<String>> {
    let mut out_aliases: HashMap<String, Vec<String>> = HashMap::new();
    for (name, cmd) in aliases {
        out_aliases
            .entry(cmd.to_string())
            .and_modify(|list| list.push(name.to_string()))
            .or_insert_with(|| vec![name.to_string()]);
    }
    out_aliases
}

impl ViewCommand for HelpCmd {
    type View = HelpView<'static>;

    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn usage(&self) -> &'static str {
        ""
    }

    fn help(&self) -> Option<HelpManual> {
        #[rustfmt::skip]
        let examples = vec![
            HelpExample::new("help",        "Open the help page for all of `explore`"),
            HelpExample::new("help :rsh",     "Open the help page for the `rsh` explore command"),
            HelpExample::new("help :help",   "...It was supposed to be hidden....until...now..."),
        ];

        #[rustfmt::skip]
        let arguments = vec![
            HelpExample::new("help :command", "you can provide a command and a help information for it will be displayed")
        ];

        Some(HelpManual {
            name: "help",
            description: "Explore the help page for `explore`",
            arguments,
            examples,
            input: vec![],
            config_options: vec![],
        })
    }

    fn parse(&mut self, args: &str) -> Result<()> {
        self.input_command = args.trim().to_owned();

        Ok(())
    }

    fn spawn(&mut self, _: &EngineState, _: &mut Stack, _: Option<Value>) -> Result<Self::View> {
        if self.input_command.is_empty() {
            return Ok(HelpView::Preview(Preview::new(Self::HELP_MESSAGE)));
        }

        if !self.input_command.starts_with(':') {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "unexpected help argument",
            ));
        }

        if self.input_command == ":" {
            let (headers, data) = help_frame_data(&self.supported_commands, &self.aliases);
            let view = RecordView::new(headers, data);
            return Ok(HelpView::Records(view));
        }

        let command = self
            .input_command
            .strip_prefix(':')
            .expect("we just checked the prefix");

        let manual = self
            .supported_commands
            .iter()
            .find(|manual| manual.name == command)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "a given command was not found"))?;

        let aliases = self
            .aliases
            .get(manual.name)
            .map(|l| l.as_slice())
            .unwrap_or(&[]);
        let (headers, data) = help_manual_data(manual, aliases);
        let view = RecordView::new(headers, data);

        Ok(HelpView::Records(view))
    }
}

fn help_frame_data(
    supported_commands: &[HelpManual],
    aliases: &HashMap<String, Vec<String>>,
) -> (Vec<String>, Vec<Vec<Value>>) {
    let commands = supported_commands
        .iter()
        .map(|manual| {
            let aliases = aliases
                .get(manual.name)
                .map(|l| l.as_slice())
                .unwrap_or(&[]);

            let (cols, mut vals) = help_manual_data(manual, aliases);
            let vals = vals.remove(0);
            Value::record(Record::from_raw_cols_vals(cols, vals), rshSpan::unknown())
        })
        .collect();
    let commands = Value::list(commands, rshSpan::unknown());

    collect_input(commands)
}

fn help_manual_data(manual: &HelpManual, aliases: &[String]) -> (Vec<String>, Vec<Vec<Value>>) {
    fn rsh_str(s: &impl ToString) -> Value {
        Value::string(s.to_string(), rshSpan::unknown())
    }

    let arguments = manual
        .arguments
        .iter()
        .map(|e| {
            Value::record(
                record! {
                    "example" => rsh_str(&e.example),
                    "description" => rsh_str(&e.description),
                },
                rshSpan::unknown(),
            )
        })
        .collect();

    let arguments = Value::list(arguments, rshSpan::unknown());

    let examples = manual
        .examples
        .iter()
        .map(|e| {
            Value::record(
                record! {
                    "example" => rsh_str(&e.example),
                    "description" => rsh_str(&e.description),
                },
                rshSpan::unknown(),
            )
        })
        .collect();
    let examples = Value::list(examples, rshSpan::unknown());

    let inputs = manual
        .input
        .iter()
        .map(|e| {
            Value::record(
                record! {
                    "name" => rsh_str(&e.code),
                    "context" => rsh_str(&e.context),
                    "description" => rsh_str(&e.description),
                },
                rshSpan::unknown(),
            )
        })
        .collect();
    let inputs = Value::list(inputs, rshSpan::unknown());

    let configuration = manual
        .config_options
        .iter()
        .map(|o| {
            let values = o
                .values
                .iter()
                .map(|v| {
                    Value::record(
                        record! {
                            "example" => rsh_str(&v.example),
                            "description" => rsh_str(&v.description),
                        },
                        rshSpan::unknown(),
                    )
                })
                .collect();
            let values = Value::list(values, rshSpan::unknown());

            Value::record(
                record! {
                    "name" => rsh_str(&o.group),
                    "context" => rsh_str(&o.key),
                    "description" => rsh_str(&o.description),
                    "values" => values,
                },
                rshSpan::unknown(),
            )
        })
        .collect();
    let configuration = Value::list(configuration, rshSpan::unknown());

    let name = rsh_str(&manual.name);
    let aliases = rsh_str(&aliases.join(", "));
    let desc = rsh_str(&manual.description);

    let headers = vec![
        String::from("name"),
        String::from("aliases"),
        String::from("arguments"),
        String::from("input"),
        String::from("examples"),
        String::from("configuration"),
        String::from("description"),
    ];

    let data = vec![vec![
        name,
        aliases,
        arguments,
        inputs,
        examples,
        configuration,
        desc,
    ]];

    (headers, data)
}
pub enum HelpView<'a> {
    Records(RecordView<'a>),
    Preview(Preview),
}

impl View for HelpView<'_> {
    fn draw(&mut self, f: &mut Frame, area: Rect, cfg: ViewConfig<'_>, layout: &mut Layout) {
        match self {
            HelpView::Records(v) => v.draw(f, area, cfg, layout),
            HelpView::Preview(v) => v.draw(f, area, cfg, layout),
        }
    }

    fn handle_input(
        &mut self,
        engine_state: &EngineState,
        stack: &mut Stack,
        layout: &Layout,
        info: &mut ViewInfo,
        key: KeyEvent,
    ) -> Option<Transition> {
        match self {
            HelpView::Records(v) => v.handle_input(engine_state, stack, layout, info, key),
            HelpView::Preview(v) => v.handle_input(engine_state, stack, layout, info, key),
        }
    }

    fn show_data(&mut self, i: usize) -> bool {
        match self {
            HelpView::Records(v) => v.show_data(i),
            HelpView::Preview(v) => v.show_data(i),
        }
    }

    fn collect_data(&self) -> Vec<crate::rsh_common::RshText> {
        match self {
            HelpView::Records(v) => v.collect_data(),
            HelpView::Preview(v) => v.collect_data(),
        }
    }

    fn exit(&mut self) -> Option<Value> {
        match self {
            HelpView::Records(v) => v.exit(),
            HelpView::Preview(v) => v.exit(),
        }
    }

    fn setup(&mut self, config: ViewConfig<'_>) {
        match self {
            HelpView::Records(v) => v.setup(config),
            HelpView::Preview(v) => v.setup(config),
        }
    }
}
