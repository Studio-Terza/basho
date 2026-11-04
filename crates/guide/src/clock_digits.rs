use std::fmt;

use chrono::{NaiveTime, TimeDelta, Timelike};

use newtype::proptest::arbitrary::any;
use newtype::proptest::strategy::Strategy;

use crate::clock::{ClockTime, Minutes};

#[derive(newtype::Newtype, newtype::Arbitrary, Clone, Copy, Debug)]
#[newtype(strategy = any::<Minutes>().prop_map(ClockTime::at_minutes))]
pub struct ClockDigits(ClockTime);

impl fmt::Display for ClockDigits {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = NaiveTime::MIN + TimeDelta::minutes(self.0.since_midnight().count());
        write!(formatter, "{}:{:02}", time.hour(), time.minute())
    }
}
