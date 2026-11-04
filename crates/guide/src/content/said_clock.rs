use std::fmt;
use std::str::FromStr;

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
}

struct SaidDigits(Minutes);

impl FromStr for SaidDigits {
    type Err = ContentError;

    fn from_str(text: &str) -> Result<SaidDigits, ContentError> {
        text.chars()
            .all(|letter| letter.is_ascii_digit())
            .then(|| text.parse().ok())
            .flatten()
            .map(|count| SaidDigits(Minutes::new(count)))
            .ok_or_else(|| ContentError::BadClock(Written::new(text.to_owned())))
    }
}

impl FromStr for SaidClock {
    type Err = ContentError;

    fn from_str(text: &str) -> Result<SaidClock, ContentError> {
        let digits = |text: &str| text.parse::<SaidDigits>().ok().map(|digits| digits.0);
        let bad = || ContentError::BadClock(Written::new(text.to_owned()));
        let (hours, minutes) = text.split_once(':').ok_or_else(bad)?;
        let hour_count = digits(hours)
            .filter(|_| (1..=Self::LONGEST_HOUR_LENGTH).contains(&hours.len()))
            .ok_or_else(bad)?;
        let minute_count = digits(minutes)
            .filter(|count| *count < Self::HOUR && minutes.len() == Self::MINUTE_LENGTH)
            .ok_or_else(bad)?;
        let padded = match (hours.len(), hours.starts_with('0')) {
            (Self::LONGEST_HOUR_LENGTH, true) => HourPadding::Zero,
            _ => HourPadding::Bare,
        };
        Some(SaidClock {
            since_midnight: Minutes::new(hour_count.count() * Self::HOUR.count()) + minute_count,
            padded,
        })
        .filter(|said| said.since_midnight <= Self::END_OF_DAY)
        .ok_or_else(bad)
    }
}

impl TryFrom<String> for SaidClock {
    type Error = ContentError;

    fn try_from(text: String) -> Result<SaidClock, ContentError> {
        text.parse()
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
