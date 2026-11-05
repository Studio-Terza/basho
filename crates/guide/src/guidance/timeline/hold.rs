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
        match leg.map(Leg::mode) {
            None => Hold::Free,
            Some(Mode::Walk) => Hold::walking(leg),
            Some(Mode::Train | Mode::Bus | Mode::Boat | Mode::Car) => Hold::Ride,
        }
    }
}

impl Hold {
    fn walking(leg: Option<&Leg>) -> Hold {
        match leg.and_then(Leg::tour) {
            Some(_) => Hold::Tour,
            None => Hold::Free,
        }
    }

    #[must_use]
    pub fn resume_after(self, closer: LegIndex) -> LegIndex {
        match self {
            Hold::Ride => closer.incremented(),
            Hold::Free | Hold::Tour => closer,
        }
    }
}
