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
    Hash,
    Deserialize,
)]
#[as_ref(str)]
#[debug("{_0:?}")]
#[serde(try_from = "String")]
pub struct StarName(String);

impl TryFrom<String> for StarName {
    type Error = NotPlain;

    fn try_from(text: String) -> Result<StarName, NotPlain> {
        PlainText::try_from(text).map(|plain| StarName(String::from(plain)))
    }
}
