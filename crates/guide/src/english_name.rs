use plain_text::{NotPlain, PlainText};
use serde::{Deserialize, Serialize};

#[derive(
    newtype::Newtype,
    newtype::Display,
    derive_more::AsRef,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
)]
#[as_ref(str)]
#[serde(try_from = "String")]
pub struct EnglishName(String);

impl TryFrom<String> for EnglishName {
    type Error = NotPlain;

    fn try_from(text: String) -> Result<EnglishName, NotPlain> {
        PlainText::try_from(text).map(|plain| EnglishName(String::from(plain)))
    }
}
