use std::fmt;

use serde::Deserialize;

use crate::content::{ContentError, Figure};
use crate::words::Written;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Price(Figure);

impl Price {
    const YEN: char = '¥';

    fn read(text: &str) -> Option<Price> {
        let rest = text.strip_prefix(Self::YEN)?;
        Figure::try_from(rest).ok().map(Price)
    }
}

impl TryFrom<String> for Price {
    type Error = ContentError;

    fn try_from(text: String) -> Result<Price, ContentError> {
        Self::read(&text).ok_or(ContentError::BadPrice(Written::new(text)))
    }
}

impl fmt::Display for Price {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", Self::YEN, self.0)
    }
}
