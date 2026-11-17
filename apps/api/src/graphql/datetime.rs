use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::{DateTime as ChronoDateTime, SecondsFormat, Utc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateTime(pub ChronoDateTime<Utc>);

#[Scalar(name = "DateTime")]
impl ScalarType for DateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => ChronoDateTime::parse_from_rfc3339(&s)
                .map(|dt| DateTime(dt.with_timezone(&Utc)))
                .map_err(|err| InputValueError::custom(format!("invalid DateTime: {err}"))),
            other => Err(InputValueError::expected_type(other)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339_opts(SecondsFormat::Secs, true))
    }
}

impl From<ChronoDateTime<Utc>> for DateTime {
    fn from(value: ChronoDateTime<Utc>) -> Self {
        Self(value)
    }
}
