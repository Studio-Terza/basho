use crate::leg::{Leg, LegIndex, Mode};
use newtype::Counter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hold {
    Free,
    Ride,
    Tour,
}

impl From<Option<&Leg>> for Hold {
    fn from(leg: Option<&Leg>) -> Hold {
        leg.filter(|leg| leg.mode() != Mode::Walk)
            .map(|_| Hold::Ride)
            .or_else(|| leg.and_then(Leg::tour).map(|_| Hold::Tour))
            .unwrap_or(Hold::Free)
    }
}

impl Hold {
    #[must_use]
    pub fn resume_after(self, closer: LegIndex) -> LegIndex {
        match self {
            Hold::Ride => closer.incremented(),
            Hold::Free | Hold::Tour => closer,
        }
    }
}
