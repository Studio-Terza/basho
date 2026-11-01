use plain_text::{NotPlain, PlainText};
use serde::Deserialize;

#[derive(
    newtype::Newtype,
    newtype::Display,
    derive_more::AsRef,
    derive_more::Debug,
    derive_more::Deref,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    serde::Serialize,
)]
#[as_ref(str)]
#[debug("{_0:?}")]
#[deref(forward)]
#[serde(try_from = "String")]
pub struct Spelling(String);

impl TryFrom<String> for Spelling {
    type Error = NotPlain;

    fn try_from(text: String) -> Result<Spelling, NotPlain> {
        PlainText::try_from(text).map(|plain| Spelling(String::from(plain)))
    }
}
