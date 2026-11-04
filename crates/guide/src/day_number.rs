use newtype::Newtype;

use serde::{Deserialize, Serialize};

#[derive(
    newtype::Display,
    Newtype,
    newtype::Arbitrary,
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
    Serialize,
)]
pub struct DayNumber(u8);

impl DayNumber {
    #[must_use]
    pub const fn number(self) -> u8 {
        self.0
    }
}

impl<Holder: AsRef<DayNumber>> From<&Holder> for DayNumber {
    fn from(holder: &Holder) -> DayNumber {
        *holder.as_ref()
    }
}
