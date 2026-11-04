use std::fmt;

use crate::clock::Minutes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shift {
    Ahead(Minutes),
    Behind(Minutes),
}

impl From<Shift> for Minutes {
    fn from(shift: Shift) -> Minutes {
        match shift {
            Shift::Ahead(early) => -early,
            Shift::Behind(late) => late,
        }
    }
}

impl fmt::Display for Shift {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shift::Ahead(early) => write!(formatter, "{early} ahead of"),
            Shift::Behind(late) => write!(formatter, "{late} behind"),
        }
    }
}
