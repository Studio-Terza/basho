use std::fmt;
use std::str::FromStr;

use serde::Deserialize;

use crate::content::{ContentError, Figure};
use crate::words::Written;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Price(Figure);

impl Price {
    const YEN: char = '¥';
}

impl FromStr for Price {
    type Err = ContentError;

    fn from_str(text: &str) -> Result<Price, ContentError> {
        text.strip_prefix(Self::YEN)
            .and_then(|rest| Figure::try_from(rest).ok())
            .map(Price)
            .ok_or_else(|| ContentError::BadPrice(Written::new(text.to_owned())))
    }
}

impl TryFrom<String> for Price {
    type Error = ContentError;

    fn try_from(text: String) -> Result<Price, ContentError> {
        text.parse()
    }
}

impl fmt::Display for Price {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", Self::YEN, self.0)
    }
}
