use rsh_engine::CallExt;
use rsh_protocol::ast::{Call, PathMember};
use rsh_protocol::engine::{Command, EngineState, Stack};
use rsh_protocol::{
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Type,
    Value,
};

#[derive(Clone)]
pub struct ToJson;

impl Command for ToJson {
    fn name(&self) -> &str {
        "to json"
    }

    fn signature(&self) -> Signature {
        Signature::build("to json")
            .input_output_types(vec![(Type::Any, Type::String)])
            .switch("raw", "remove all of the whitespace", Some('r'))
            .named(
                "indent",
                SyntaxShape::Number,
                "specify indentation width",
                Some('i'),
            )
            .named(
                "tabs",
                SyntaxShape::Number,
                "specify indentation tab quantity",
                Some('t'),
            )
            .category(Category::Formats)
    }

    fn usage(&self) -> &str {
        "Converts table data into JSON text."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let raw = call.has_flag("raw");
        let use_tabs = call.has_flag("tabs");

        let span = call.head;
        // allow ranges to expand and turn into array
        let input = input.try_expand_range()?;
        let value = input.into_value(span);
        let json_value = value_to_json_value(&value)?;

        let json_result = if raw {
            rsh_json::to_string_raw(&json_value)
        } else if use_tabs {
            let tab_count: usize = call.get_flag(engine_state, stack, "tabs")?.unwrap_or(1);
            rsh_json::to_string_with_tab_indentation(&json_value, tab_count)
        } else {
            let indent: usize = call.get_flag(engine_state, stack, "indent")?.unwrap_or(2);
            rsh_json::to_string_with_indent(&json_value, indent)
        };

        match json_result {
            Ok(serde_json_string) => {
                Ok(Value::string(serde_json_string, span).into_pipeline_data())
            }
            _ => Ok(Value::error(
                ShellError::CantConvert {
                    to_type: "JSON".into(),
                    from_type: value.get_type().to_string(),
                    span,
                    help: None,
                },
                span,
            )
            .into_pipeline_data()),
        }
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description:
                    "Outputs a JSON string, with default indentation, representing the contents of this table",
                example: "[a b c] | to json",
                result: Some(Value::test_string("[\n  \"a\",\n  \"b\",\n  \"c\"\n]")),
            },
            Example {
                description:
                    "Outputs a JSON string, with 4-space indentation, representing the contents of this table",
                example: "[Joe Bob Sam] | to json --indent 4",
                result: Some(Value::test_string("[\n    \"Joe\",\n    \"Bob\",\n    \"Sam\"\n]")),
            },
            Example {
                description:
                    "Outputs an unformatted JSON string representing the contents of this table",
                example: "[1 2 3] | to json -r",
                result: Some(Value::test_string("[1,2,3]")),
            },
        ]
    }
}

pub fn value_to_json_value(v: &Value) -> Result<rsh_json::Value, ShellError> {
    let span = v.span();
    Ok(match v {
        Value::Bool { val, .. } => rsh_json::Value::Bool(*val),
        Value::Filesize { val, .. } => rsh_json::Value::I64(*val),
        Value::Duration { val, .. } => rsh_json::Value::I64(*val),
        Value::Date { val, .. } => rsh_json::Value::String(val.to_string()),
        Value::Float { val, .. } => rsh_json::Value::F64(*val),
        Value::Int { val, .. } => rsh_json::Value::I64(*val),
        Value::Nothing { .. } => rsh_json::Value::Null,
        Value::String { val, .. } => rsh_json::Value::String(val.to_string()),
        Value::CellPath { val, .. } => rsh_json::Value::Array(
            val.members
                .iter()
                .map(|x| match &x {
                    PathMember::String { val, .. } => Ok(rsh_json::Value::String(val.clone())),
                    PathMember::Int { val, .. } => Ok(rsh_json::Value::U64(*val as u64)),
                })
                .collect::<Result<Vec<rsh_json::Value>, ShellError>>()?,
        ),

        Value::List { vals, .. } => rsh_json::Value::Array(json_list(vals)?),
        Value::Error { error, .. } => return Err(*error.clone()),
        Value::Closure { .. }
        | Value::Block { .. }
        | Value::Range { .. }
        | Value::MatchPattern { .. } => rsh_json::Value::Null,
        Value::Binary { val, .. } => {
            rsh_json::Value::Array(val.iter().map(|x| rsh_json::Value::U64(*x as u64)).collect())
        }
        Value::Record { val, .. } => {
            let mut m = rsh_json::Map::new();
            for (k, v) in val {
                m.insert(k.clone(), value_to_json_value(v)?);
            }
            rsh_json::Value::Object(m)
        }
        Value::LazyRecord { val, .. } => {
            let collected = val.collect()?;
            value_to_json_value(&collected)?
        }
        Value::CustomValue { val, .. } => {
            let collected = val.to_base_value(span)?;
            value_to_json_value(&collected)?
        }
    })
}

fn json_list(input: &[Value]) -> Result<Vec<rsh_json::Value>, ShellError> {
    let mut out = vec![];

    for value in input {
        out.push(value_to_json_value(value)?);
    }

    Ok(out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(ToJson {})
    }
}
