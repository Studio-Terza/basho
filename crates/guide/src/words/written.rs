use plain_text::{NotPlain, PlainText};
use serde::Deserialize;

#[derive(
    newtype::Newtype,
    newtype::Display,
    derive_more::AsRef,
    derive_more::Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
)]
#[as_ref(str)]
#[debug("{_0:?}")]
#[serde(try_from = "String")]
pub struct Written(String);

impl TryFrom<String> for Written {
    type Error = NotPlain;

    fn try_from(text: String) -> Result<Written, NotPlain> {
        PlainText::try_from(text).map(|plain| Written(String::from(plain)))
    }
}
