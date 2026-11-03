use std::borrow::Borrow;
use std::fmt;
use std::str::FromStr;

use serde::Deserialize;

mod not_plain;

pub use not_plain::NotPlain;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct PlainText(String);

impl PlainText {
    const KEPT: [char; 3] = ['\n', '\r', '\t'];
}

impl FromStr for PlainText {
    type Err = NotPlain;

    fn from_str(text: &str) -> Result<PlainText, NotPlain> {
        if text
            .chars()
            .any(|letter| letter.is_ascii_control() && !Self::KEPT.contains(&letter))
        {
            Err(NotPlain)
        } else {
            Ok(PlainText(text.to_owned()))
        }
    }
}

impl TryFrom<String> for PlainText {
    type Error = NotPlain;

    fn try_from(text: String) -> Result<PlainText, NotPlain> {
        text.parse()
    }
}

impl From<PlainText> for String {
    fn from(text: PlainText) -> String {
        text.0
    }
}

impl fmt::Display for PlainText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl AsRef<str> for PlainText {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for PlainText {
    fn borrow(&self) -> &str {
        &self.0
    }
}
