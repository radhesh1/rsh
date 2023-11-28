use super::super::super::values::{Column, rshDataFrame};

use rsh_engine::CallExt;
use rsh_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, PipelineData, ShellError, Signature, Span, SyntaxShape, Type, Value,
};
use polars::prelude::{ChunkSet, DataType, IntoSeries};

#[derive(Clone)]
pub struct SetWithIndex;

impl Command for SetWithIndex {
    fn name(&self) -> &str {
        "dfr set-with-idx"
    }

    fn usage(&self) -> &str {
        "Sets value in the given index."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("value", SyntaxShape::Any, "value to be inserted in series")
            .required_named(
                "indices",
                SyntaxShape::Any,
                "list of indices indicating where to set the value",
                Some('i'),
            )
            .input_output_type(
                Type::Custom("dataframe".into()),
                Type::Custom("dataframe".into()),
            )
            .category(Category::Custom("dataframe".into()))
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Set value in selected rows from series",
            example: r#"let series = ([4 1 5 2 4 3] | dfr into-df);
    let indices = ([0 2] | dfr into-df);
    $series | dfr set-with-idx 6 --indices $indices"#,
            result: Some(
                rshDataFrame::try_from_columns(vec![Column::new(
                    "0".to_string(),
                    vec![
                        Value::test_int(6),
                        Value::test_int(1),
                        Value::test_int(6),
                        Value::test_int(2),
                        Value::test_int(4),
                        Value::test_int(3),
                    ],
                )])
                .expect("simple df for test should not fail")
                .into_value(Span::test_data()),
            ),
        }]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        command(engine_state, stack, call, input)
    }
}

fn command(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let value: Value = call.req(engine_state, stack, 0)?;

    let indices_value: Value = call
        .get_flag(engine_state, stack, "indices")?
        .expect("required named value");
    let indices_span = indices_value.span();
    let indices = rshDataFrame::try_from_value(indices_value)?.as_series(indices_span)?;

    let casted = match indices.dtype() {
        DataType::UInt32 | DataType::UInt64 | DataType::Int32 | DataType::Int64 => {
            indices.as_ref().cast(&DataType::UInt32).map_err(|e| {
                ShellError::GenericError(
                    "Error casting indices".into(),
                    e.to_string(),
                    Some(indices_span),
                    None,
                    Vec::new(),
                )
            })
        }
        _ => Err(ShellError::GenericError(
            "Incorrect type".into(),
            "Series with incorrect type".into(),
            Some(indices_span),
            Some("Consider using a Series with type int type".into()),
            Vec::new(),
        )),
    }?;

    let indices = casted
        .u32()
        .map_err(|e| {
            ShellError::GenericError(
                "Error casting indices".into(),
                e.to_string(),
                Some(indices_span),
                None,
                Vec::new(),
            )
        })?
        .into_iter()
        .flatten();

    let df = rshDataFrame::try_from_pipeline(input, call.head)?;
    let series = df.as_series(call.head)?;

    let span = value.span();
    let res = match value {
        Value::Int { val, .. } => {
            let chunked = series.i64().map_err(|e| {
                ShellError::GenericError(
                    "Error casting to i64".into(),
                    e.to_string(),
                    Some(span),
                    None,
                    Vec::new(),
                )
            })?;

            let res = chunked.set_at_idx(indices, Some(val)).map_err(|e| {
                ShellError::GenericError(
                    "Error setting value".into(),
                    e.to_string(),
                    Some(span),
                    None,
                    Vec::new(),
                )
            })?;

            rshDataFrame::try_from_series(vec![res.into_series()], call.head)
        }
        Value::Float { val, .. } => {
            let chunked = series.f64().map_err(|e| {
                ShellError::GenericError(
                    "Error casting to f64".into(),
                    e.to_string(),
                    Some(span),
                    None,
                    Vec::new(),
                )
            })?;

            let res = chunked.set_at_idx(indices, Some(val)).map_err(|e| {
                ShellError::GenericError(
                    "Error setting value".into(),
                    e.to_string(),
                    Some(span),
                    None,
                    Vec::new(),
                )
            })?;

            rshDataFrame::try_from_series(vec![res.into_series()], call.head)
        }
        Value::String { val, .. } => {
            let chunked = series.utf8().map_err(|e| {
                ShellError::GenericError(
                    "Error casting to string".into(),
                    e.to_string(),
                    Some(span),
                    None,
                    Vec::new(),
                )
            })?;

            let res = chunked
                .set_at_idx(indices, Some(val.as_ref()))
                .map_err(|e| {
                    ShellError::GenericError(
                        "Error setting value".into(),
                        e.to_string(),
                        Some(span),
                        None,
                        Vec::new(),
                    )
                })?;

            let mut res = res.into_series();
            res.rename("string");

            rshDataFrame::try_from_series(vec![res.into_series()], call.head)
        }
        _ => Err(ShellError::GenericError(
            "Incorrect value type".into(),
            format!(
                "this value cannot be set in a series of type '{}'",
                series.dtype()
            ),
            Some(span),
            None,
            Vec::new(),
        )),
    };

    res.map(|df| PipelineData::Value(rshDataFrame::into_value(df, call.head), None))
}

#[cfg(test)]
mod test {
    use super::super::super::super::test_dataframe::test_dataframe;
    use super::*;

    #[test]
    fn test_examples() {
        test_dataframe(vec![Box::new(SetWithIndex {})])
    }
}
