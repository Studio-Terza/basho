use std::convert::Infallible;
use std::iter;
use std::str::FromStr;

#[derive(newtype::Display, derive_more::Deref, Clone, Debug, PartialEq, Eq)]
#[deref(forward)]
pub struct CssValue(String);

impl FromStr for CssValue {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<CssValue, Infallible> {
        Ok(CssValue(text.trim().to_owned()))
    }
}

impl CssValue {
    const VAR: &str = "var(";

    #[must_use]
    pub fn has_literal(&self) -> bool {
        let head = self.0.split(Self::VAR).next().unwrap_or_default();
        let tails = self
            .0
            .split(Self::VAR)
            .skip(1)
            .map(|piece| piece.split_once(')').map_or("", |(_, after)| after));
        iter::once(head)
            .chain(tails)
            .any(|piece| piece.chars().any(|character| character.is_ascii_digit()))
    }
}
