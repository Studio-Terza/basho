use std::fmt;
use std::str::FromStr;

use crate::text_failure::TextFailure;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bearer(String);

impl Bearer {
    const TOKEN_PREFIX: &str = "eyJ";
}

impl FromStr for Bearer {
    type Err = TextFailure;

    fn from_str(transcript: &str) -> Result<Bearer, TextFailure> {
        transcript
            .split_whitespace()
            .find(|word| word.starts_with(Self::TOKEN_PREFIX))
            .map(|token| Bearer(token.to_owned()))
            .ok_or(TextFailure::NoToken)
    }
}

impl fmt::Display for Bearer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Authorization: Bearer {}", self.0)
    }
}
