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
    const WORDS: usize = 2;

    #[must_use]
    pub fn new(leg: LegIndex, stage: Stage) -> Placing {
        Placing { leg, stage }
    }

    #[must_use]
    pub(super) fn from_words(words: &[&str]) -> Option<Placing> {
        words
            .split_first_chunk::<{ Self::WORDS }>()
            .and_then(|([leg, stage], _)| {
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
            Stage::Boarding => {
                (clock - leg.time(End::Start).since_midnight() + Minutes::new(1)).max(Minutes::ZERO)
            }
            Stage::Under => Some(leg.time(End::Finish).since_midnight())
                .filter(|end| clock >= *end)
                .map_or(Minutes::ZERO, |_| {
                    clock - leg.time(End::Start).since_midnight()
                }),
            Stage::Finished => Minutes::ZERO,
        }
    }
}

impl fmt::Display for Placing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.leg, self.stage)
    }
}
