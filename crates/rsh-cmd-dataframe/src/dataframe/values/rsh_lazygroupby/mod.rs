mod custom_value;

use core::fmt;
use rsh_protocol::{PipelineData, ShellError, Span, Value};
use polars::prelude::{LazyGroupBy, Schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Lazyframe wrapper for rsh operations
// Polars LazyFrame is behind and Option to allow easy implementation of
// the Deserialize trait
#[derive(Default)]
pub struct rshLazyGroupBy {
    pub group_by: Option<LazyGroupBy>,
    pub schema: Option<Schema>,
    pub from_eager: bool,
}

// Mocked serialization of the LazyFrame object
impl Serialize for rshLazyGroupBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

// Mocked deserialization of the LazyFrame object
impl<'de> Deserialize<'de> for rshLazyGroupBy {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(rshLazyGroupBy::default())
    }
}

impl fmt::Debug for rshLazyGroupBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rshLazyGroupBy")
    }
}

// Referenced access to the real LazyFrame
impl AsRef<LazyGroupBy> for rshLazyGroupBy {
    fn as_ref(&self) -> &polars::prelude::LazyGroupBy {
        // The only case when there cannot be a lazy frame is if it is created
        // using the default function or if created by deserializing something
        self.group_by
            .as_ref()
            .expect("there should always be a frame")
    }
}

impl AsMut<LazyGroupBy> for rshLazyGroupBy {
    fn as_mut(&mut self) -> &mut polars::prelude::LazyGroupBy {
        // The only case when there cannot be a lazy frame is if it is created
        // using the default function or if created by deserializing something
        self.group_by
            .as_mut()
            .expect("there should always be a frame")
    }
}

impl From<LazyGroupBy> for rshLazyGroupBy {
    fn from(group_by: LazyGroupBy) -> Self {
        Self {
            group_by: Some(group_by),
            from_eager: false,
            schema: None,
        }
    }
}

impl rshLazyGroupBy {
    pub fn into_value(self, span: Span) -> Value {
        Value::custom_value(Box::new(self), span)
    }

    pub fn into_polars(self) -> LazyGroupBy {
        self.group_by.expect("GroupBy cannot be none to convert")
    }

    pub fn try_from_value(value: Value) -> Result<Self, ShellError> {
        let span = value.span();
        match value {
            Value::CustomValue { val, .. } => match val.as_any().downcast_ref::<rshLazyGroupBy>() {
                Some(group) => Ok(Self {
                    group_by: group.group_by.clone(),
                    schema: group.schema.clone(),
                    from_eager: group.from_eager,
                }),
                None => Err(ShellError::CantConvert {
                    to_type: "lazy groupby".into(),
                    from_type: "custom value".into(),
                    span,
                    help: None,
                }),
            },
            x => Err(ShellError::CantConvert {
                to_type: "lazy groupby".into(),
                from_type: x.get_type().to_string(),
                span: x.span(),
                help: None,
            }),
        }
    }

    pub fn try_from_pipeline(input: PipelineData, span: Span) -> Result<Self, ShellError> {
        let value = input.into_value(span);
        Self::try_from_value(value)
    }
}
