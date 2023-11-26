use heck::ToShoutySnakeCase;
use rsh_protocol::ast::Call;
use rsh_protocol::engine::{Command, EngineState, Stack};
use rsh_protocol::{
    record, Category, Example, PipelineData, ShellError, Signature, SyntaxShape, Type, Value,
};

use super::operate;

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "str screaming-snake-case"
    }

    fn signature(&self) -> Signature {
        Signature::build("str screaming-snake-case")
            .input_output_types(vec![
                (Type::String, Type::String),
                (
                    Type::List(Box::new(Type::String)),
                    Type::List(Box::new(Type::String)),
                ),
                (Type::Table(vec![]), Type::Table(vec![])),
                (Type::Record(vec![]), Type::Record(vec![])),
            ])
            .allow_variants_without_examples(true)
            .rest(
                "rest",
                SyntaxShape::CellPath,
                "For a data structure input, convert strings at the given cell paths",
            )
            .category(Category::Strings)
    }

    fn usage(&self) -> &str {
        "Convert a string to SCREAMING_SNAKE_CASE."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["convert", "style", "underscore", "convention"]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        operate(
            engine_state,
            stack,
            call,
            input,
            &ToShoutySnakeCase::to_shouty_snake_case,
        )
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "convert a string to SCREAMING_SNAKE_CASE",
                example: r#" "rsh" | str screaming-snake-case"#,
                result: Some(Value::test_string("RSH")),
            },
            Example {
                description: "convert a string to SCREAMING_SNAKE_CASE",
                example: r#" "this_is_the_second_case" | str screaming-snake-case"#,
                result: Some(Value::test_string("THIS_IS_THE_SECOND_CASE")),
            },
            Example {
                description: "convert a string to SCREAMING_SNAKE_CASE",
                example: r#""this-is-the-first-case" | str screaming-snake-case"#,
                result: Some(Value::test_string("THIS_IS_THE_FIRST_CASE")),
            },
            Example {
                description: "convert a column from a table to SCREAMING_SNAKE_CASE",
                example: r#"[[lang, gems]; [rsh_test, 100]] | str screaming-snake-case lang"#,
                result: Some(Value::test_list(vec![Value::test_record(record! {
                    "lang" =>  Value::test_string("rsh_TEST"),
                    "gems" =>  Value::test_int(100),
                })])),
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }
}
