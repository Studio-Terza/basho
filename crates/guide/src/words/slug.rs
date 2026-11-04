use plain_text::{NotPlain, PlainText};
use serde::Deserialize;

#[derive(
    newtype::Newtype,
    newtype::Arbitrary,
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
#[newtype(strategy = "\\PC{0,24}")]
pub struct Slug(String);

impl TryFrom<String> for Slug {
    type Error = NotPlain;

    fn try_from(text: String) -> Result<Slug, NotPlain> {
        PlainText::try_from(text).map(|plain| Slug(String::from(plain)))
    }
}
