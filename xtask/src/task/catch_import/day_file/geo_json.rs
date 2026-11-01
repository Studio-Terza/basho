use newtype::Newtype;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Newtype, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GeoJson(Value);

impl GeoJson {
    #[must_use]
    pub fn json(&self) -> &Value {
        &self.0
    }

    #[must_use]
    pub fn into_value(self) -> Value {
        self.0
    }

    #[must_use]
    pub fn without_nulls(self) -> GeoJson {
        GeoJson(Self::pruned(&self.0))
    }

    fn pruned(value: &Value) -> Value {
        match value {
            Value::Object(fields) => Value::Object(
                fields
                    .iter()
                    .filter(|(_, field)| !field.is_null())
                    .map(|(name, field)| (name.clone(), Self::pruned(field)))
                    .collect(),
            ),
            Value::Array(items) => Value::Array(items.iter().map(Self::pruned).collect()),
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => value.clone(),
        }
    }
}
