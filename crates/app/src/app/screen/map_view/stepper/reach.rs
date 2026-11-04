use dioxus::dioxus_core::{AttributeValue, IntoAttributeValue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reach {
    Open,
    End,
}

impl Reach {
    pub fn of_target<Target>(target: Option<Target>) -> Reach {
        target.map_or(Reach::End, |_| Reach::Open)
    }
}

impl IntoAttributeValue for Reach {
    fn into_value(self) -> AttributeValue {
        match self {
            Reach::Open => AttributeValue::None,
            Reach::End => AttributeValue::Bool(true),
        }
    }
}
