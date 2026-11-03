use std::ops::Add;

use serde::Serialize;

#[derive(
    newtype::Newtype, newtype::Display, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize,
)]
#[serde(transparent)]
pub struct FactCount(usize);

impl Add for FactCount {
    type Output = FactCount;

    fn add(self, other: FactCount) -> FactCount {
        FactCount(self.0 + other.0)
    }
}
