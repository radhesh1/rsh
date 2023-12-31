use chrono::Local;
use rsh_protocol::ast::Call;
use rsh_protocol::engine::{Command, EngineState, Stack};
use rsh_protocol::{
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Type, Value,
};
#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "date now"
    }

    fn signature(&self) -> Signature {
        Signature::build("date now")
            .input_output_types(vec![(Type::Nothing, Type::Date)])
            .category(Category::Date)
    }

    fn usage(&self) -> &str {
        "Get the current date."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["present", "current-time"]
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let head = call.head;
        let dt = Local::now();
        Ok(Value::date(dt.with_timezone(dt.offset()), head).into_pipeline_data())
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Get the current date and display it in a given format string.",
                example: r#"date now | format date "%Y-%m-%d %H:%M:%S""#,
                result: None,
            },
            Example {
                description: "Get the time duration from 2023-04-30 to now",
                example: r#"(date now) - 2023-11-25"#,
                result: None,
            },
            Example {
                description: "Get the time duration since a more accurate time",
                example: r#"(date now) - 2023-11-25T04:12:05.20+05:30"#,
                result: None,
            },
            Example {
                description: "Get current time in full RFC3339 format with timezone",
                example: r#"date now | debug"#,
                result: None,
            },
        ]
    }
}
