use std::fmt;

use crate::clock::Minutes;
use crate::times::listed_time::ListedTime;
use crate::times::shift::Shift;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Basis {
    PlanTimes,
    PlanLeft(Minutes),
    PlanAfterRide,
    OffPlan { listed: ListedTime, shift: Shift },
}

impl fmt::Display for Basis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Basis::PlanTimes => formatter.write_str("by plan times"),
            Basis::PlanLeft(left) => write!(formatter, "{left} of plan left"),
            Basis::PlanAfterRide => formatter.write_str("plan after it"),
            Basis::OffPlan { listed, shift } => write!(formatter, "{shift} {listed}"),
        }
    }
}
