use serde::Deserialize;

#[derive(newtype::Display, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct ShortName(String);

impl ShortName {
    const MAX_LENGTH: usize = 12;
}

impl TryFrom<String> for ShortName {
    type Error = ShortNameError;

    fn try_from(text: String) -> Result<ShortName, ShortNameError> {
        if text.chars().count() > Self::MAX_LENGTH {
            Err(ShortNameError(format!(
                "the night's short name {text:?} is over {} characters",
                Self::MAX_LENGTH
            )))
        } else {
            Ok(ShortName(text))
        }
    }
}

#[derive(newtype::Display, Debug)]
pub struct ShortNameError(String);
