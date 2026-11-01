use std::borrow::Cow;

use guide::{Trip, TripError};
use plain_text::{NotPlain, PlainText};
use serde::Deserialize;

#[derive(
    newtype::Newtype, newtype::Display, derive_more::Debug, Clone, PartialEq, Eq, Deserialize,
)]
#[debug("{} bytes of trip text", _0.len())]
#[serde(try_from = "String")]
pub struct TripText(Cow<'static, str>);

impl TripText {
    pub fn trip(&self) -> Result<Trip, TripError> {
        self.0.parse()
    }
}

impl TryFrom<String> for TripText {
    type Error = NotPlain;

    fn try_from(text: String) -> Result<TripText, NotPlain> {
        PlainText::try_from(text).map(|plain| TripText(Cow::Owned(String::from(plain))))
    }
}
