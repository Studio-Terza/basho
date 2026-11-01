use dioxus_core::{AttributeValue, IntoAttributeValue};

#[derive(newtype::Display, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassName(&'static str);

impl<Source: Into<&'static str>> From<Source> for ClassName {
    fn from(source: Source) -> ClassName {
        ClassName(source.into())
    }
}

impl IntoAttributeValue for ClassName {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.0.to_owned())
    }
}
