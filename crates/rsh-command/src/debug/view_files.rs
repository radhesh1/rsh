use rsh_protocol::ast::Call;
use rsh_protocol::engine::{Command, EngineState, Stack};
use rsh_protocol::{
    record, Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Type, Value,
};

#[derive(Clone)]
pub struct ViewFiles;

impl Command for ViewFiles {
    fn name(&self) -> &str {
        "view files"
    }

    fn usage(&self) -> &str {
        "View the files registered in rsh's EngineState memory."
    }

    fn extra_usage(&self) -> &str {
        "These are files parsed and loaded at runtime."
    }

    fn signature(&self) -> rsh_protocol::Signature {
        Signature::build("view files")
            .input_output_types(vec![(
                Type::Nothing,
                Type::Table(vec![
                    ("filename".into(), Type::String),
                    ("start".into(), Type::Int),
                    ("end".into(), Type::Int),
                    ("size".into(), Type::Int),
                ]),
            )])
            .category(Category::Debug)
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let mut records = vec![];

        for (file, start, end) in engine_state.files() {
            records.push(Value::record(
                record! {
                    "filename" => Value::string(file, call.head),
                    "start" => Value::int(*start as i64, call.head),
                    "end" => Value::int(*end as i64, call.head),
                    "size" => Value::int(*end as i64 - *start as i64, call.head),
                },
                call.head,
            ));
        }

        Ok(Value::list(records, call.head).into_pipeline_data())
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "View the files registered in Rsh's EngineState memory",
                example: r#"view files"#,
                result: None,
            },
            Example {
                description: "View how Rsh was originally invoked",
                example: r#"view files | get 0"#,
                result: None,
            },
        ]
    }
}
