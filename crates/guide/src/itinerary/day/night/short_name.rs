use serde::Deserialize;
use std::cmp::Ordering;

#[derive(newtype::Display, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct ShortName(String);

impl ShortName {
    const MAX_LENGTH: usize = 12;
}

impl TryFrom<String> for ShortName {
    type Error = ShortNameError;

    fn try_from(text: String) -> Result<ShortName, ShortNameError> {
        match text.chars().count().cmp(&Self::MAX_LENGTH) {
            Ordering::Greater => Err(ShortNameError(format!(
                "the night's short name {text:?} is over {} characters",
                Self::MAX_LENGTH
            ))),
            Ordering::Less | Ordering::Equal => Ok(ShortName(text)),
        }
    }
}

#[derive(newtype::Display, Debug)]
pub struct ShortNameError(String);
