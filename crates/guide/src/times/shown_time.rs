use std::cmp::Ordering;
use std::fmt;

use crate::classed::Classed;
use crate::clock::{ClockTime, Minutes};
use crate::times::basis::Basis;
use crate::times::fact_time::FactTime;
use crate::times::listed_time::ListedTime;
use crate::times::live_time::LiveTime;
use crate::times::plan_time::PlanTime;
use crate::times::shift::Shift;

#[derive(derive_more::From, strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShownTime {
    #[strum(serialize = "time-fact")]
    Stated(FactTime),
    #[strum(serialize = "time-plan")]
    Planned(PlanTime),
    #[strum(serialize = "time-live")]
    Live(LiveTime),
}

impl Classed for ShownTime {}

impl ShownTime {
    #[must_use]
    pub fn off_by(listed: ListedTime, by: Minutes) -> ShownTime {
        match by.cmp(&Minutes::ZERO) {
            Ordering::Equal => ShownTime::from(listed),
            Ordering::Less => ShownTime::shifted(listed, Shift::Ahead(by.abs())),
            Ordering::Greater => ShownTime::shifted(listed, Shift::Behind(by)),
        }
    }

    fn shifted(listed: ListedTime, shift: Shift) -> ShownTime {
        ShownTime::Live(LiveTime::new(
            ClockTime::from(listed).later_by(Minutes::from(shift)),
            Basis::OffPlan { listed, shift },
        ))
    }
}

impl From<ShownTime> for ClockTime {
    fn from(time: ShownTime) -> ClockTime {
        match time {
            ShownTime::Stated(fact) => ClockTime::from(fact),
            ShownTime::Planned(plan) => ClockTime::from(plan),
            ShownTime::Live(live) => ClockTime::from(live),
        }
    }
}

impl fmt::Display for ShownTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShownTime::Stated(fact) => fact.fmt(formatter),
            ShownTime::Planned(plan) => plan.fmt(formatter),
            ShownTime::Live(live) => live.fmt(formatter),
        }
    }
}
