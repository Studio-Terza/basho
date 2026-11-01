use crate::words::Written;

use chrono::{NaiveTime, Timelike};
use serde::Deserialize;

use crate::clock::clock_error::ClockError;
use crate::clock::minutes::Minutes;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(try_from = "String")]
pub struct ClockTime(u16);

impl ClockTime {
    pub const MIDNIGHT: ClockTime = ClockTime(0);
    const MINUTES_PER_HOUR: u16 = 60;
    const MINUTES_PER_DAY: u16 = 24 * 60;
    const SECONDS_PER_MINUTE: u32 = 60;
    const WRITTEN: &str = "%H:%M";

    #[must_use]
    pub const fn at_hour(hours: u16) -> ClockTime {
        ClockTime(hours * Self::MINUTES_PER_HOUR % Self::MINUTES_PER_DAY)
    }

    #[must_use]
    pub fn at_minutes(minutes: Minutes) -> ClockTime {
        ClockTime::MIDNIGHT.later_by(minutes)
    }

    #[must_use]
    pub fn since_midnight(self) -> Minutes {
        Minutes::new(i64::from(self.0))
    }

    #[must_use]
    pub fn minutes_until(self, later: ClockTime) -> Minutes {
        later.since_midnight() - self.since_midnight()
    }

    #[must_use]
    pub fn later_by(self, minutes: Minutes) -> ClockTime {
        let day = i64::from(Self::MINUTES_PER_DAY);
        ClockTime(
            u16::try_from((self.since_midnight().count() + minutes.count()).rem_euclid(day))
                .unwrap_or_default(),
        )
    }

    pub(super) fn naive(self) -> NaiveTime {
        NaiveTime::from_num_seconds_from_midnight_opt(
            u32::from(self.0) * Self::SECONDS_PER_MINUTE,
            0,
        )
        .unwrap_or_default()
    }
}

impl From<NaiveTime> for ClockTime {
    fn from(time: NaiveTime) -> ClockTime {
        ClockTime::at_minutes(Minutes::new(i64::from(
            time.num_seconds_from_midnight() / Self::SECONDS_PER_MINUTE,
        )))
    }
}

impl TryFrom<String> for ClockTime {
    type Error = ClockError;

    fn try_from(text: String) -> Result<ClockTime, ClockError> {
        NaiveTime::parse_from_str(&text, Self::WRITTEN)
            .map(ClockTime::from)
            .map_err(|_| ClockError::Time(Written::new(text)))
    }
}
