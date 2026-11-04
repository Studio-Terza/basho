use newtype::Newtype;
use newtype::proptest::arbitrary::any;
use newtype::proptest::collection::vec;
use newtype::proptest::strategy::Strategy;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Newtype, newtype::Arbitrary, Clone, Debug, PartialEq, Deserialize, Serialize)]
#[newtype(strategy = vec(any::<u16>(), 0..GeoJson::MOST_FEATURES).prop_map(|ids| json!({ "features": ids })))]
pub struct GeoJson(Value);

impl GeoJson {
    const MOST_FEATURES: usize = 4;

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
