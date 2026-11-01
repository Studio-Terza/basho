use std::fmt;
use std::str::FromStr;

use guide::itinerary::Day;

use guide::guidance::LegProgress;

pub mod legs_stamp;
mod unreadable;

use legs_stamp::LegsStamp;
pub use unreadable::Unreadable;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeptProgress {
    progress: LegProgress,
    legs: LegsStamp,
}

impl KeptProgress {
    const STAMP: &str = "legs";

    #[must_use]
    pub fn new(progress: LegProgress, day: &Day) -> KeptProgress {
        KeptProgress {
            progress,
            legs: LegsStamp::taken_from(day),
        }
    }

    fn stamp_in(line: &str) -> Result<LegsStamp, Unreadable> {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [word, legs] if *word == Self::STAMP => legs.parse().map_err(|_| Unreadable::Stamp),
            _ => Err(Unreadable::Stamp),
        }
    }

    #[must_use]
    pub fn for_day(self, day: &Day) -> Option<LegProgress> {
        (self.legs == LegsStamp::taken_from(day))
            .then_some(self.progress)
            .and_then(|progress| progress.for_day(day.number()))
    }
}

impl fmt::Display for KeptProgress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{}", self.progress)?;
        writeln!(formatter, "{} {}", Self::STAMP, self.legs)
    }
}

impl FromStr for KeptProgress {
    type Err = Unreadable;

    fn from_str(text: &str) -> Result<KeptProgress, Unreadable> {
        match text.lines().collect::<Vec<&str>>().as_slice() {
            [progress, stamp] => Ok(KeptProgress {
                progress: progress.parse().map_err(|_| Unreadable::Progress)?,
                legs: Self::stamp_in(stamp)?,
            }),
            _ => Err(Unreadable::Lines),
        }
    }
}
