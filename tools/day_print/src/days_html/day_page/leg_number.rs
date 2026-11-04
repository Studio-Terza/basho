use std::iter;

use guide::leg::LegIndex;
use newtype::Counter;

#[derive(newtype::Display, Clone, Copy)]
pub struct LegNumber(usize);

impl LegNumber {
    #[must_use]
    pub fn from_index(index: LegIndex) -> LegNumber {
        LegNumber(usize::from(index.incremented()))
    }

    pub fn onward(self) -> impl Iterator<Item = LegNumber> {
        iter::successors(Some(self), |number| Some(LegNumber(number.0 + 1)))
    }
}
