use std::fmt;

use serde::Deserialize;

use crate::clock::Minutes;
use crate::content::ContentError;
use crate::words::Written;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct SaidClock {
    since_midnight: Minutes,
    padded: HourPadding,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HourPadding {
    Bare,
    Zero,
}

impl SaidClock {
    const END_OF_DAY: Minutes = Minutes::new(24 * 60);
    const HOUR: Minutes = Minutes::new(60);
    const LONGEST_HOUR_LENGTH: usize = 2;
    const MINUTE_LENGTH: usize = 2;

    fn digits(text: &str) -> Option<Minutes> {
        text.parse()
            .ok()
            .filter(|_| text.chars().all(|letter| letter.is_ascii_digit()))
            .map(Minutes::new)
    }

    pub(super) fn read(text: &str) -> Option<SaidClock> {
        let (hours, minutes) = text.split_once(':')?;
        let hour_count = Self::digits(hours)
            .filter(|_| (1..=Self::LONGEST_HOUR_LENGTH).contains(&hours.len()))?;
        let minute_count = Self::digits(minutes)
            .filter(|count| *count < Self::HOUR && minutes.len() == Self::MINUTE_LENGTH)?;
        let padded = match (hours.len(), hours.starts_with('0')) {
            (Self::LONGEST_HOUR_LENGTH, true) => HourPadding::Zero,
            _ => HourPadding::Bare,
        };
        Some(SaidClock {
            since_midnight: Minutes::new(hour_count.count() * Self::HOUR.count()) + minute_count,
            padded,
        })
        .filter(|said| said.since_midnight <= Self::END_OF_DAY)
    }
}

impl TryFrom<String> for SaidClock {
    type Error = ContentError;

    fn try_from(text: String) -> Result<SaidClock, ContentError> {
        Self::read(&text).ok_or(ContentError::BadClock(Written::new(text)))
    }
}

impl fmt::Display for SaidClock {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.since_midnight.count() / Self::HOUR.count();
        let minutes = self.since_midnight.count() % Self::HOUR.count();
        match self.padded {
            HourPadding::Bare => write!(formatter, "{hours}:{minutes:02}"),
            HourPadding::Zero => write!(formatter, "{hours:02}:{minutes:02}"),
        }
    }
}
