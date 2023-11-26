use super::rshWhen;
use rsh_protocol::{CustomValue, ShellError, Span, Value};

// CustomValue implementation for rshDataFrame
impl CustomValue for rshWhen {
    fn typetag_name(&self) -> &'static str {
        "when"
    }

    fn typetag_deserialize(&self) {
        unimplemented!("typetag_deserialize")
    }

    fn clone_value(&self, span: rsh_protocol::Span) -> Value {
        let cloned = self.clone();

        Value::custom_value(Box::new(cloned), span)
    }

    fn value_string(&self) -> String {
        self.typetag_name().to_string()
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        let val: String = match self {
            rshWhen::Then(_) => "whenthen".into(),
            rshWhen::ChainedThen(_) => "whenthenthen".into(),
        };

        let value = Value::string(val, span);
        Ok(value)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
