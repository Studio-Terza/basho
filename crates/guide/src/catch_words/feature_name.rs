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
    Deserialize,
)]
#[as_ref(str)]
#[debug("{_0:?}")]
#[serde(try_from = "String")]
#[newtype(strategy = "\\PC{0,24}")]
pub struct FeatureName(String);

impl TryFrom<String> for FeatureName {
    type Error = NotPlain;

    fn try_from(text: String) -> Result<FeatureName, NotPlain> {
        PlainText::try_from(text).map(|plain| FeatureName(String::from(plain)))
    }
}
