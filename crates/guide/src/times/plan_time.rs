use std::fmt;

use serde::Deserialize;

use newtype::proptest::arbitrary::any;
use newtype::proptest::strategy::Strategy;

use crate::clock::{ClockTime, Minutes};
use crate::clock_digits::ClockDigits;

#[derive(
    newtype::Newtype,
    newtype::Arbitrary,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
)]
#[newtype(strategy = any::<Minutes>().prop_map(ClockTime::at_minutes))]
#[serde(deny_unknown_fields)]
pub struct PlanTime {
    plan: ClockTime,
}

impl From<PlanTime> for ClockTime {
    fn from(time: PlanTime) -> ClockTime {
        time.plan
    }
}

impl fmt::Display for PlanTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "plan {}", ClockDigits::new(self.plan))
    }
}
