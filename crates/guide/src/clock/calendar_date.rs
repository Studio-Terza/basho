use crate::words::{Spelling, Written};
use std::fmt;

use chrono::NaiveDate;
use serde::Deserialize;

use crate::clock::clock_error::ClockError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct CalendarDate(NaiveDate);

impl CalendarDate {
    const WRITTEN: &str = "%Y-%m-%d";
    const SHORT: &str = "%a %b %-d";

    #[must_use]
    pub fn is_day_after(self, previous: CalendarDate) -> Option<CalendarDate> {
        previous
            .0
            .succ_opt()
            .filter(|next| *next == self.0)
            .map(|_| self)
    }

    #[must_use]
    pub fn short(self) -> Spelling {
        Spelling::new(self.0.format(Self::SHORT).to_string())
    }
}

impl From<NaiveDate> for CalendarDate {
    fn from(date: NaiveDate) -> CalendarDate {
        CalendarDate(date)
    }
}

impl From<CalendarDate> for NaiveDate {
    fn from(date: CalendarDate) -> NaiveDate {
        date.0
    }
}

impl TryFrom<String> for CalendarDate {
    type Error = ClockError;

    fn try_from(text: String) -> Result<CalendarDate, ClockError> {
        NaiveDate::parse_from_str(&text, Self::WRITTEN)
            .map(CalendarDate)
            .map_err(|_| ClockError::Date(Written::new(text)))
    }
}

impl fmt::Display for CalendarDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0.format(Self::WRITTEN))
    }
}
