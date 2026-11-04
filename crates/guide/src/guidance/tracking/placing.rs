use std::fmt;
use std::str::FromStr;

use thiserror::Error;

use super::stage::Stage;
use crate::clock::Minutes;
use crate::end::End;
use crate::leg::{Leg, LegIndex};
use crate::words::Written;

#[derive(Debug, Error, PartialEq, Eq)]
#[error("{0:?} isn't a leg number and a stage")]
pub struct PlacingError(Written);

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
        let start = leg.time(End::Start).since_midnight();
        let end = leg.time(End::Finish).since_midnight();
        match self.stage {
            Stage::Boarding => (clock - start + Minutes::new(1)).max(Minutes::ZERO),
            Stage::Under if clock >= end => clock - start,
            Stage::Under | Stage::Finished => Minutes::ZERO,
        }
    }
}

impl FromStr for Placing {
    type Err = PlacingError;

    fn from_str(text: &str) -> Result<Placing, PlacingError> {
        <[&str; Placing::WORDS]>::try_from(
            text.split_whitespace()
                .take(Placing::WORDS)
                .collect::<Vec<_>>(),
        )
        .ok()
        .and_then(|[leg, stage]| {
            Some(Placing::new(
                LegIndex::new(leg.parse::<usize>().ok()?.checked_sub(1)?),
                stage.parse().ok()?,
            ))
        })
        .ok_or_else(|| PlacingError(Written::new(text.to_owned())))
    }
}

impl fmt::Display for Placing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.leg, self.stage)
    }
}
