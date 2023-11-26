use rsh_protocol::ast::Call;
use rsh_protocol::engine::{Command, EngineState, Stack};
use rsh_protocol::{Category, Example, PipelineData, ShellError, Signature, SyntaxShape, Type};

#[derive(Clone)]
pub struct Register;

impl Command for Register {
    fn name(&self) -> &str {
        "register"
    }

    fn usage(&self) -> &str {
        "Register a plugin."
    }

    fn signature(&self) -> rsh_protocol::Signature {
        Signature::build("register")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .required(
                "plugin",
                SyntaxShape::Filepath,
                "path of executable for plugin",
            )
            .optional(
                "signature",
                SyntaxShape::Any,
                "Block with signature description as json object",
            )
            .named(
                "shell",
                SyntaxShape::Filepath,
                "path of shell used to run plugin (cmd, sh, python, etc)",
                Some('s'),
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
        vec![
            Example {
                description: "Register `rsh_plugin_query` plugin from ~/.cargo/bin/ dir",
                example: r#"register ~/.cargo/bin/rsh_plugin_query"#,
                result: None,
            },
            Example {
                description: "Register `rsh_plugin_query` plugin from `rsh -c` (writes/updates $rsh.plugin-path)",
                example: r#"let plugin = ((which rsh).path.0 | path dirname | path join 'rsh_plugin_query'); rsh -c $'register ($plugin); version'"#,
                result: None,
            },
        ]
    }
}
