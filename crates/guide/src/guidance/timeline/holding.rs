use crate::clock::Minutes;
use crate::times::{ListedTime, ShownTime};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Holding {
    Held,
    Free,
}

impl From<Ordering> for Holding {
    fn from(against_closer: Ordering) -> Holding {
        match against_closer {
            Ordering::Less => Holding::Held,
            Ordering::Equal | Ordering::Greater => Holding::Free,
        }
    }
}

impl Holding {
    #[must_use]
    pub fn shown(self, planned: ListedTime, offset: Minutes) -> ShownTime {
        match self {
            Holding::Held => ShownTime::off_by(planned, offset),
            Holding::Free => ShownTime::from(planned),
        }
    }
}
