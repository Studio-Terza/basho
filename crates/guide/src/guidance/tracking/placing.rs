use std::cmp::Ordering;
use std::fmt;

use super::stage::Stage;
use crate::clock::Minutes;
use crate::end::End;
use crate::leg::{Leg, LegIndex};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Placing {
    leg: LegIndex,
    stage: Stage,
}

impl Placing {
    #[must_use]
    pub fn new(leg: LegIndex, stage: Stage) -> Placing {
        Placing { leg, stage }
    }

    #[must_use]
    pub(super) fn from_words(words: &[&str]) -> Option<Placing> {
        words.first().zip(words.get(1)).and_then(|(leg, stage)| {
            Some(Placing::new(
                LegIndex::new(leg.parse::<usize>().ok()?.checked_sub(1)?),
                stage.parse().ok()?,
            ))
        })
    }

    #[must_use]
    pub fn on_leg(self) -> LegIndex {
        self.leg
    }

    #[must_use]
    pub fn stage(self) -> Stage {
        self.stage
    }
}

impl Placing {
    #[must_use]
    pub fn lateness(self, leg: &Leg, clock: Minutes) -> Minutes {
        match self.stage {
            Stage::Boarding => Self::boarding_lateness(leg, clock),
            Stage::Under => Self::under_lateness(leg, clock),
            Stage::Finished => Minutes::ZERO,
        }
    }

    fn boarding_lateness(leg: &Leg, clock: Minutes) -> Minutes {
        (clock - leg.time(End::Start).since_midnight() + Minutes::new(1)).max(Minutes::ZERO)
    }

    fn under_lateness(leg: &Leg, clock: Minutes) -> Minutes {
        match clock.cmp(&leg.time(End::Finish).since_midnight()) {
            Ordering::Less => Minutes::ZERO,
            Ordering::Equal | Ordering::Greater => clock - leg.time(End::Start).since_midnight(),
        }
    }
}

impl fmt::Display for Placing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.leg, self.stage)
    }
}
