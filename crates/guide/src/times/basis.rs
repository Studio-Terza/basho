use std::fmt;

use crate::clock::Minutes;
use crate::times::listed_time::ListedTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Basis {
    PlanTimes,
    PlanLeft(Minutes),
    PlanAfterRide,
    OffPlan { listed: ListedTime, by: Minutes },
}

impl fmt::Display for Basis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Basis::PlanTimes => formatter.write_str("by plan times"),
            Basis::PlanLeft(left) => write!(formatter, "{left} of plan left"),
            Basis::PlanAfterRide => formatter.write_str("plan after it"),
            Basis::OffPlan { listed, by } => Self::off_plan(formatter, *listed, *by),
        }
    }
}

impl Basis {
    fn off_plan(
        formatter: &mut fmt::Formatter<'_>,
        listed: ListedTime,
        by: Minutes,
    ) -> fmt::Result {
        formatter.write_str(&Some(by).filter(|by| *by < Minutes::ZERO).map_or_else(
            || format!("{by} behind {listed}"),
            |by| format!("{} ahead of {listed}", by.abs()),
        ))
    }
}
