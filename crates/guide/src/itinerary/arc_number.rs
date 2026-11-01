use newtype::Newtype;

use serde::Deserialize;

#[derive(
    newtype::Display,
    Newtype,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
)]
pub struct ArcNumber(u8);

impl ArcNumber {
    #[must_use]
    pub const fn number(self) -> u8 {
        self.0
    }

    #[must_use]
    pub fn counted_from_one(self) -> u16 {
        u16::from(self.0) + 1
    }
}
