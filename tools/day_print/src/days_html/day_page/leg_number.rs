use std::iter;

use guide::leg::LegIndex;

#[derive(newtype::Display, Clone, Copy)]
pub struct LegNumber(usize);

impl LegNumber {
    pub fn onward(self) -> impl Iterator<Item = LegNumber> {
        iter::successors(Some(self), |number| Some(LegNumber(number.0 + 1)))
    }
}

impl From<LegIndex> for LegNumber {
    fn from(index: LegIndex) -> LegNumber {
        LegNumber(usize::from(index) + 1)
    }
}
