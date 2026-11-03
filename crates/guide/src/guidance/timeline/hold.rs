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
        match leg {
            Some(leg) if leg.mode() != Mode::Walk => Hold::Ride,
            Some(leg) if leg.tour().is_some() => Hold::Tour,
            Some(_) | None => Hold::Free,
        }
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
