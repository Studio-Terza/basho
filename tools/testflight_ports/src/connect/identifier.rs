use serde::Deserialize;
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
#[error(
    "an App Store Connect identifier is letters, digits, hyphens and underscores, and is never empty"
)]
pub struct NotIdentifier;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Identifier(String);

impl Identifier {
    const JOINERS: [char; 2] = ['-', '_'];
}

impl TryFrom<String> for Identifier {
    type Error = NotIdentifier;

    fn try_from(text: String) -> Result<Identifier, NotIdentifier> {
        if !text.is_empty()
            && text.chars().all(|letter| {
                letter.is_ascii_alphanumeric() || Identifier::JOINERS.contains(&letter)
            })
        {
            Ok(Identifier(text))
        } else {
            Err(NotIdentifier)
        }
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
