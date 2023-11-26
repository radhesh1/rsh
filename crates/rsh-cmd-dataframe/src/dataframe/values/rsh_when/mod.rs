mod custom_value;

use core::fmt;
use rsh_protocol::{ShellError, Span, Value};
use polars::prelude::{col, when, ChainedThen, Then};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone)]
pub enum rshWhen {
    Then(Box<Then>),
    ChainedThen(ChainedThen),
}

// Mocked serialization of the LazyFrame object
impl Serialize for rshWhen {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

// Mocked deserialization of the LazyFrame object
impl<'de> Deserialize<'de> for rshWhen {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(rshWhen::Then(Box::new(when(col("a")).then(col("b")))))
    }
}

impl fmt::Debug for rshWhen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rshWhen")
    }
}

impl From<Then> for rshWhen {
    fn from(then: Then) -> Self {
        rshWhen::Then(Box::new(then))
    }
}

impl From<ChainedThen> for rshWhen {
    fn from(chained_when: ChainedThen) -> Self {
        rshWhen::ChainedThen(chained_when)
    }
}

impl rshWhen {
    pub fn into_value(self, span: Span) -> Value {
        Value::custom_value(Box::new(self), span)
    }

    pub fn try_from_value(value: Value) -> Result<Self, ShellError> {
        let span = value.span();
        match value {
            Value::CustomValue { val, .. } => match val.as_any().downcast_ref::<Self>() {
                Some(expr) => Ok(expr.clone()),
                None => Err(ShellError::CantConvert {
                    to_type: "when expression".into(),
                    from_type: "non when expression".into(),
                    span,
                    help: None,
                }),
            },
            x => Err(ShellError::CantConvert {
                to_type: "when expression".into(),
                from_type: x.get_type().to_string(),
                span: x.span(),
                help: None,
            }),
        }
    }
}
