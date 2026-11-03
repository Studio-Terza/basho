use std::fmt;

use chrono::{NaiveTime, TimeDelta, Timelike};

use crate::clock::ClockTime;

#[derive(newtype::Newtype, Clone, Copy, Debug)]
pub struct ClockDigits(ClockTime);

impl fmt::Display for ClockDigits {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = NaiveTime::MIN + TimeDelta::minutes(self.0.since_midnight().count());
        write!(formatter, "{}:{:02}", time.hour(), time.minute())
    }
}
