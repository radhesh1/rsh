use super::rshLazyGroupBy;
use rsh_protocol::{record, CustomValue, ShellError, Span, Value};

// CustomValue implementation for rshDataFrame
impl CustomValue for rshLazyGroupBy {
    fn typetag_name(&self) -> &'static str {
        "lazygroupby"
    }

    fn typetag_deserialize(&self) {
        unimplemented!("typetag_deserialize")
    }

    fn clone_value(&self, span: rsh_protocol::Span) -> Value {
        let cloned = rshLazyGroupBy {
            group_by: self.group_by.clone(),
            schema: self.schema.clone(),
            from_eager: self.from_eager,
        };

        Value::custom_value(Box::new(cloned), span)
    }

    fn value_string(&self) -> String {
        self.typetag_name().to_string()
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        Ok(Value::record(
            record! {
                "LazyGroupBy" => Value::string("apply aggregation to complete execution plan", span)
            },
            span,
        ))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
