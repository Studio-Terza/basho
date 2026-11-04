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
        let (amount, word) = Some(self.spare)
            .filter(|spare| *spare < Minutes::ZERO)
            .map_or((self.spare, "to spare"), |spare| (spare.abs(), "short"));
        write!(formatter, "{amount} {word} · {}", self.basis)
    }
}
