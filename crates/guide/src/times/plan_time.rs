use std::fmt;

use serde::Deserialize;

use crate::clock::ClockTime;
use crate::clock_digits::ClockDigits;

#[derive(newtype::Newtype, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
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
