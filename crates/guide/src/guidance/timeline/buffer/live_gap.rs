use std::fmt;

use crate::clock::Minutes;
use crate::times::Basis;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiveGap {
    spare: Minutes,
    basis: Basis,
}

impl LiveGap {
    #[must_use]
    pub fn new(spare: Minutes, basis: Basis) -> LiveGap {
        LiveGap { spare, basis }
    }
}

impl fmt::Display for LiveGap {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.spare < Minutes::ZERO {
            write!(formatter, "{} short · {}", self.spare.abs(), self.basis)
        } else {
            write!(formatter, "{} to spare · {}", self.spare, self.basis)
        }
    }
}
