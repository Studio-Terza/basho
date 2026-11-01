use newtype::Newtype;
use std::fmt;
use std::num::NonZeroUsize;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(
    derive_more::Into, Newtype, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct LegIndex(usize);

impl LegIndex {
    #[must_use]
    pub const fn number(self) -> NonZeroUsize {
        NonZeroUsize::MIN.saturating_add(self.0)
    }

    #[must_use]
    pub fn previous(self) -> Option<LegIndex> {
        self.0.checked_sub(1).map(LegIndex)
    }
}

impl<'de> Deserialize<'de> for LegIndex {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<LegIndex, D::Error> {
        usize::deserialize(deserializer).and_then(|number| {
            number
                .checked_sub(1)
                .map(LegIndex)
                .ok_or_else(|| D::Error::custom("leg numbers start at 1"))
        })
    }
}

impl fmt::Display for LegIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.number())
    }
}
