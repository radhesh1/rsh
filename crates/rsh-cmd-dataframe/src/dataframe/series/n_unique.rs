use super::super::values::{Column, RshDataFrame, RshExpression};
use rsh_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, PipelineData, ShellError, Signature, Span, Type, Value,
};

#[derive(Clone)]
pub struct NUnique;

impl Command for NUnique {
    fn name(&self) -> &str {
        "dfr n-unique"
    }

    fn usage(&self) -> &str {
        "Counts unique values."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![
                (
                    Type::Custom("expression".into()),
                    Type::Custom("expression".into()),
                ),
                (
                    Type::Custom("dataframe".into()),
                    Type::Custom("dataframe".into()),
                ),
            ])
            .category(Category::Custom("dataframe".into()))
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Counts unique values",
                example: "[1 1 2 2 3 3 4] | dfr into-df | dfr n-unique",
                result: Some(
                    RshDataFrame::try_from_columns(vec![Column::new(
                        "count_unique".to_string(),
                        vec![Value::test_int(4)],
                    )])
                    .expect("simple df for test should not fail")
                    .into_value(Span::test_data()),
                ),
            },
            Example {
                description: "Creates a is n-unique expression from a column",
                example: "dfr col a | dfr n-unique",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let value = input.into_value(call.head);
        if RshDataFrame::can_downcast(&value) {
            let df = RshDataFrame::try_from_value(value)?;
            command(engine_state, stack, call, df)
        } else {
            let expr = RshExpression::try_from_value(value)?;
            let expr: RshExpression = expr.into_polars().n_unique().into();

            Ok(PipelineData::Value(
                RshExpression::into_value(expr, call.head),
                None,
            ))
        }
    }
}

fn command(
    _engine_state: &EngineState,
    _stack: &mut Stack,
    call: &Call,
    df: RshDataFrame,
) -> Result<PipelineData, ShellError> {
    let res = df.as_series(call.head)?.n_unique().map_err(|e| {
        ShellError::GenericError(
            "Error counting unique values".into(),
            e.to_string(),
            Some(call.head),
            None,
            Vec::new(),
        )
    })?;

    let value = Value::int(res as i64, call.head);

    RshDataFrame::try_from_columns(vec![Column::new("count_unique".to_string(), vec![value])])
        .map(|df| PipelineData::Value(RshDataFrame::into_value(df, call.head), None))
}

#[cfg(test)]
mod test {
    use super::super::super::test_dataframe::{build_test_engine_state, test_dataframe_example};
    use super::*;
    use crate::dataframe::lazy::aggregate::LazyAggregate;
    use crate::dataframe::lazy::groupby::ToLazyGroupBy;

    #[test]
    fn test_examples_dataframe() {
        let mut engine_state = build_test_engine_state(vec![Box::new(NUnique {})]);
        test_dataframe_example(&mut engine_state, &NUnique.examples()[0]);
    }

    #[test]
    fn test_examples_expression() {
        let mut engine_state = build_test_engine_state(vec![
            Box::new(NUnique {}),
            Box::new(LazyAggregate {}),
            Box::new(ToLazyGroupBy {}),
        ]);
        test_dataframe_example(&mut engine_state, &NUnique.examples()[1]);
    }
}
