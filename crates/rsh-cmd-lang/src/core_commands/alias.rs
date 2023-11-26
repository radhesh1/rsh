use rsh_protocol::ast::Call;
use rsh_protocol::engine::{Command, EngineState, Stack};
use rsh_protocol::{
    Category, Example, PipelineData, ShellError, Signature, Span, SyntaxShape, Type, Value,
};

#[derive(Clone)]
pub struct Alias;

impl Command for Alias {
    fn name(&self) -> &str {
        "alias"
    }

    fn usage(&self) -> &str {
        "Alias a command (with optional flags) to a new name."
    }

    fn signature(&self) -> rsh_protocol::Signature {
        Signature::build("alias")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .required("name", SyntaxShape::String, "name of the alias")
            .required(
                "initial_value",
                SyntaxShape::Keyword(b"=".to_vec(), Box::new(SyntaxShape::Expression)),
                "equals sign followed by value",
            )
            .category(Category::Core)
    }

    fn extra_usage(&self) -> &str {
        r#"This command is a parser keyword. For details, check:
  https://irsh.eu.org/book/thinking_in_rsh.html"#
    }

    fn is_parser_keyword(&self) -> bool {
        true
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["abbr", "aka", "fn", "func", "function"]
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        _call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        Ok(PipelineData::empty())
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Alias ll to ls -l",
            example: "alias ll = ls -l",
            result: Some(Value::nothing(Span::test_data())),
        }]
    }
}
