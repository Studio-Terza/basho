mod css_value;

use std::str::FromStr;

use basho_lints::Failure;
pub use css_value::CssValue;

#[derive(
    newtype::Newtype,
    newtype::Arbitrary,
    newtype::Display,
    derive_more::Deref,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[deref(forward)]
pub struct CssProperty(String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Declaration {
    property: CssProperty,
    value: CssValue,
}

impl Declaration {
    #[must_use]
    pub fn property(&self) -> &CssProperty {
        &self.property
    }

    #[must_use]
    pub fn assigned(&self) -> &CssValue {
        &self.value
    }
}

impl FromStr for Declaration {
    type Err = Failure;

    fn from_str(text: &str) -> Result<Declaration, Failure> {
        text.split_once(':')
            .map(|(property, value)| Declaration {
                property: CssProperty::new(property.trim().to_owned()),
                value: value.parse().unwrap_or_else(|never| match never {}),
            })
            .ok_or(Failure::Unassigned)
    }
}
