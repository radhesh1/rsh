use rsh_engine::{eval_block, CallExt};
use rsh_protocol::ast::{Call, CellPath, PathMember};
use rsh_protocol::engine::{Closure, Command, EngineState, Stack};
use rsh_protocol::{
    record, Category, Example, FromValue, IntoInterruptiblePipelineData, IntoPipelineData,
    PipelineData, ShellError, Signature, SyntaxShape, Type, Value,
};

#[derive(Clone)]
pub struct Update;

impl Command for Update {
    fn name(&self) -> &str {
        "update"
    }

    fn signature(&self) -> Signature {
        Signature::build("update")
            .input_output_types(vec![
                (Type::Record(vec![]), Type::Record(vec![])),
                (Type::Table(vec![]), Type::Table(vec![])),
                (
                    Type::List(Box::new(Type::Any)),
                    Type::List(Box::new(Type::Any)),
                ),
            ])
            .required(
                "field",
                SyntaxShape::CellPath,
                "the name of the column to update",
            )
            .required(
                "replacement value",
                SyntaxShape::Any,
                "the new value to give the cell(s), or a closure to create the value",
            )
            .allow_variants_without_examples(true)
            .category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Update an existing column to have a new value."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        update(engine_state, stack, call, input)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Update a column value",
                example: "{'name': 'rsh', 'stars': 5} | update name 'Rsh'",
                result: Some(Value::test_record(record! {
                    "name" =>  Value::test_string("Rsh"),
                    "stars" => Value::test_int(5),
                })),
            },
            Example {
                description: "Use in closure form for more involved updating logic",
                example: "[[count fruit]; [1 'apple']] | enumerate | update item.count {|e| ($e.item.fruit | str length) + $e.index } | get item",
                result: Some(Value::test_list(
                    vec![Value::test_record(record! {
                        "count" => Value::test_int(5),
                        "fruit" => Value::test_string("apple"),
                    })],
                )),
            },
            Example {
                description: "Alter each value in the 'authors' column to use a single string instead of a list",
                example: "[[project, authors]; ['rsh', ['Andrés', 'JT', 'Yehuda']]] | update authors {|row| $row.authors | str join ','}",
                result: Some(Value::test_list(
                    vec![Value::test_record(record! {
                        "project" => Value::test_string("rsh"),
                        "authors" => Value::test_string("Andrés,JT,Yehuda"),
                    })],
                )),
            },
            Example {
                description: "You can also use a simple command to update 'authors' to a single string",
                example: "[[project, authors]; ['rsh', ['Andrés', 'JT', 'Yehuda']]] | update authors {|| str join ','}",
                result: Some(Value::test_list(
                    vec![Value::test_record(record! {
                        "project" => Value::test_string("rsh"),
                        "authors" => Value::test_string("Andrés,JT,Yehuda"),
                    })],
                )),
            }
        ]
    }
}

fn update(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let span = call.head;

    let cell_path: CellPath = call.req(engine_state, stack, 0)?;
    let replacement: Value = call.req(engine_state, stack, 1)?;

    let redirect_stdout = call.redirect_stdout;
    let redirect_stderr = call.redirect_stderr;

    let engine_state = engine_state.clone();
    let ctrlc = engine_state.ctrlc.clone();

    // Let's capture the metadata for ls_colors
    let metadata = input.metadata();
    let mdclone = metadata.clone();

    // Replace is a block, so set it up and run it instead of using it as the replacement
    if replacement.as_block().is_ok() {
        let capture_block = Closure::from_value(replacement)?;
        let block = engine_state.get_block(capture_block.block_id).clone();

        let mut stack = stack.captures_to_stack(capture_block.captures);
        let orig_env_vars = stack.env_vars.clone();
        let orig_env_hidden = stack.env_hidden.clone();

        Ok(input
            .map(
                move |mut input| {
                    // with_env() is used here to ensure that each iteration uses
                    // a different set of environment variables.
                    // Hence, a 'cd' in the first loop won't affect the next loop.
                    stack.with_env(&orig_env_vars, &orig_env_hidden);

                    if let Some(var) = block.signature.get_positional(0) {
                        if let Some(var_id) = &var.var_id {
                            stack.add_var(*var_id, input.clone())
                        }
                    }

                    let input_at_path =
                        match input.clone().follow_cell_path(&cell_path.members, false) {
                            Err(e) => return Value::error(e, span),
                            Ok(v) => v,
                        };
                    let output = eval_block(
                        &engine_state,
                        &mut stack,
                        &block,
                        input_at_path.into_pipeline_data_with_metadata(metadata.clone()),
                        redirect_stdout,
                        redirect_stderr,
                    );

                    match output {
                        Ok(pd) => {
                            if let Err(e) = input
                                .update_data_at_cell_path(&cell_path.members, pd.into_value(span))
                            {
                                return Value::error(e, span);
                            }

                            input
                        }
                        Err(e) => Value::error(e, span),
                    }
                },
                ctrlc,
            )?
            .set_metadata(mdclone))
    } else {
        if let Some(PathMember::Int { val, span, .. }) = cell_path.members.first() {
            let mut input = input.into_iter();
            let mut pre_elems = vec![];

            for idx in 0..*val {
                if let Some(v) = input.next() {
                    pre_elems.push(v);
                } else if idx == 0 {
                    return Err(ShellError::AccessEmptyContent { span: *span });
                } else {
                    return Err(ShellError::AccessBeyondEnd {
                        max_idx: idx - 1,
                        span: *span,
                    });
                }
            }

            // Skip over the replaced value
            let _ = input.next();

            return Ok(pre_elems
                .into_iter()
                .chain(vec![replacement])
                .chain(input)
                .into_pipeline_data_with_metadata(metadata, ctrlc));
        }
        Ok(input
            .map(
                move |mut input| {
                    let replacement = replacement.clone();

                    if let Err(e) = input.update_data_at_cell_path(&cell_path.members, replacement)
                    {
                        return Value::error(e, span);
                    }

                    input
                },
                ctrlc,
            )?
            .set_metadata(metadata))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(Update {})
    }
}
