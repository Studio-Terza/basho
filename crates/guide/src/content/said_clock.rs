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

impl HourPadding {
    fn written(self, formatter: &mut fmt::Formatter<'_>, said: SaidClock) -> fmt::Result {
        match self {
            HourPadding::Bare => write!(
                formatter,
                "{}:{:02}",
                said.since_midnight.count() / SaidClock::HOUR.count(),
                said.since_midnight.count() % SaidClock::HOUR.count()
            ),
            HourPadding::Zero => write!(
                formatter,
                "{:02}:{:02}",
                said.since_midnight.count() / SaidClock::HOUR.count(),
                said.since_midnight.count() % SaidClock::HOUR.count()
            ),
        }
    }
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
            .filter(|count| *count < Self::HOUR)
            .filter(|_| minutes.len() == Self::MINUTE_LENGTH)?;
        let padded = Some(hours)
            .filter(|hours| hours.len() == Self::LONGEST_HOUR_LENGTH)
            .filter(|hours| hours.starts_with('0'))
            .map_or(HourPadding::Bare, |_| HourPadding::Zero);
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
        self.padded.written(formatter, *self)
    }
}
