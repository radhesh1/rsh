use rsh_protocol::ast::Call;
use rsh_protocol::engine::{Command, EngineState, Stack};
use rsh_protocol::{
    Category, Example, PipelineData, ShellError, Signature, SyntaxShape, Type, Value,
};

#[derive(Clone)]
pub struct ExportUse;

impl Command for ExportUse {
    fn name(&self) -> &str {
        "export use"
    }

    fn usage(&self) -> &str {
        "Use definitions from a module and export them from this module."
    }

    fn signature(&self) -> rsh_protocol::Signature {
        Signature::build("export use")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .required("module", SyntaxShape::String, "Module or module file")
            .optional(
                "members",
                SyntaxShape::Any,
                "Which members of the module to import",
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
            description: "Re-export a command from another module",
            example: r#"module spam { export def foo [] { "foo" } }
    module eggs { export use spam foo }
    use eggs foo
    foo
            "#,
            result: Some(Value::test_string("foo")),
        }]
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["reexport", "import", "module"]
    }
}
