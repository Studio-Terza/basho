use std::iter;

#[derive(newtype::Display, Clone, Copy)]
pub struct LegNumber(usize);

impl LegNumber {
    #[must_use]
    pub fn from_index(index: usize) -> LegNumber {
        LegNumber(index + 1)
    }

    pub fn onward(self) -> impl Iterator<Item = LegNumber> {
        iter::successors(Some(self), |number| Some(LegNumber(number.0 + 1)))
    }
}
