use rsh_protocol::ast::Call;
use rsh_protocol::engine::{Command, EngineState, Stack};
use rsh_protocol::{Category, Example, PipelineData, ShellError, Signature, SyntaxShape, Type};

#[derive(Clone)]
pub struct Const;

impl Command for Const {
    fn name(&self) -> &str {
        "const"
    }

    fn usage(&self) -> &str {
        "Create a parse-time constant."
    }

    fn signature(&self) -> rsh_protocol::Signature {
        Signature::build("const")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .allow_variants_without_examples(true)
            .required("const_name", SyntaxShape::VarWithOptType, "constant name")
            .required(
                "initial_value",
                SyntaxShape::Keyword(b"=".to_vec(), Box::new(SyntaxShape::MathExpression)),
                "equals sign followed by constant value",
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
        vec!["set", "let"]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let var_id = if let Some(id) = call.positional_nth(0).and_then(|pos| pos.as_var()) {
            id
        } else {
            return Err(ShellError::RshFailedSpanned {
                msg: "Could not get variable".to_string(),
                label: "variable not added by the parser".to_string(),
                span: call.head,
            });
        };

        if let Some(constval) = &engine_state.get_var(var_id).const_val {
            stack.add_var(var_id, constval.clone());

            Ok(PipelineData::empty())
        } else {
            Err(ShellError::RshFailedSpanned {
                msg: "Missing Constant".to_string(),
                label: "constant not added by the parser".to_string(),
                span: call.head,
            })
        }
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Create a new parse-time constant.",
                example: "const x = 10",
                result: None,
            },
            Example {
                description: "Create a composite constant value",
                example: "const x = { a: 10, b: 20 }",
                result: None,
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use rsh_protocol::engine::CommandType;

    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(Const {})
    }

    #[test]
    fn test_command_type() {
        assert!(matches!(Const.command_type(), CommandType::Keyword));
    }
}
