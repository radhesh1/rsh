use super::super::super::values::{Column, rshDataFrame, rshExpression};
use rsh_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, PipelineData, ShellError, Signature, Span, Type, Value,
};
use polars::prelude::IntoSeries;

#[derive(Clone)]
pub struct IsNotNull;

impl Command for IsNotNull {
    fn name(&self) -> &str {
        "dfr is-not-null"
    }

    fn usage(&self) -> &str {
        "Creates mask where value is not null."
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
                description: "Create mask where values are not null",
                example: r#"let s = ([5 6 0 8] | dfr into-df);
    let res = ($s / $s);
    $res | dfr is-not-null"#,
                result: Some(
                    rshDataFrame::try_from_columns(vec![Column::new(
                        "is_not_null".to_string(),
                        vec![
                            Value::test_bool(true),
                            Value::test_bool(true),
                            Value::test_bool(false),
                            Value::test_bool(true),
                        ],
                    )])
                    .expect("simple df for test should not fail")
                    .into_value(Span::test_data()),
                ),
            },
            Example {
                description: "Creates a is not null expression from a column",
                example: "dfr col a | dfr is-not-null",
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
        if rshDataFrame::can_downcast(&value) {
            let df = rshDataFrame::try_from_value(value)?;
            command(engine_state, stack, call, df)
        } else {
            let expr = rshExpression::try_from_value(value)?;
            let expr: rshExpression = expr.into_polars().is_not_null().into();

            Ok(PipelineData::Value(
                rshExpression::into_value(expr, call.head),
                None,
            ))
        }
    }
}

fn command(
    _engine_state: &EngineState,
    _stack: &mut Stack,
    call: &Call,
    df: rshDataFrame,
) -> Result<PipelineData, ShellError> {
    let mut res = df.as_series(call.head)?.is_not_null();
    res.rename("is_not_null");

    rshDataFrame::try_from_series(vec![res.into_series()], call.head)
        .map(|df| PipelineData::Value(rshDataFrame::into_value(df, call.head), None))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dataframe::lazy::aggregate::LazyAggregate;
    use crate::dataframe::lazy::groupby::ToLazyGroupBy;
    use crate::dataframe::test_dataframe::{build_test_engine_state, test_dataframe_example};

    #[test]
    fn test_examples_dataframe() {
        let mut engine_state = build_test_engine_state(vec![Box::new(IsNotNull {})]);
        test_dataframe_example(&mut engine_state, &IsNotNull.examples()[0]);
    }

    #[test]
    fn test_examples_expression() {
        let mut engine_state = build_test_engine_state(vec![
            Box::new(IsNotNull {}),
            Box::new(LazyAggregate {}),
            Box::new(ToLazyGroupBy {}),
        ]);
        test_dataframe_example(&mut engine_state, &IsNotNull.examples()[1]);
    }
}
