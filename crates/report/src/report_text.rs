use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

use plain_text::{NotPlain, PlainText};
use serde::{Deserialize, Serialize};

#[derive(
    newtype::Newtype,
    newtype::Arbitrary,
    newtype::Display,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
)]
#[serde(try_from = "String")]
pub struct ReportText(String);

impl ReportText {
    #[must_use]
    pub fn rendered(shown: &impl fmt::Display) -> ReportText {
        ReportText(shown.to_string())
    }
}

impl FromStr for ReportText {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<ReportText, Infallible> {
        Ok(ReportText(text.to_owned()))
    }
}

impl TryFrom<String> for ReportText {
    type Error = NotPlain;

    fn try_from(text: String) -> Result<ReportText, NotPlain> {
        PlainText::try_from(text).map(|plain| ReportText(String::from(plain)))
    }
}
