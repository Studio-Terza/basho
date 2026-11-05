pub mod allowance;

use std::cmp::Ordering;
use std::fmt;

use crate::clock::{Minutes, TripTime};
use crate::geo::Distance;
use crate::leg::Leg;
use crate::readout::Emphasis;
use crate::times::{FactTime, ShownTime};
use crate::words::Spelling;
use allowance::Allowance;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeCheck {
    Expected {
        by: ShownTime,
        left: Minutes,
    },
    Estimated {
        by: ShownTime,
        left: Minutes,
        from: Option<FactTime>,
    },
    Due {
        by: ShownTime,
        over: Minutes,
    },
    Late {
        by: ShownTime,
        over: Minutes,
    },
}

impl TimeCheck {
    #[must_use]
    pub fn comparing(due: TripTime, by: ShownTime, now: TripTime, leg: &Leg) -> TimeCheck {
        Self::overrun(now.minutes_since(due), by, leg)
    }

    fn overrun(over: Minutes, by: ShownTime, leg: &Leg) -> TimeCheck {
        match over.cmp(&Minutes::ZERO) {
            Ordering::Less | Ordering::Equal => TimeCheck::Expected { by, left: -over },
            Ordering::Greater => Self::allowed(over.cmp(&Allowance::for_leg(leg)), by, over),
        }
    }

    fn allowed(against_allowance: Ordering, by: ShownTime, over: Minutes) -> TimeCheck {
        match against_allowance {
            Ordering::Less | Ordering::Equal => TimeCheck::Due { by, over },
            Ordering::Greater => TimeCheck::Late { by, over },
        }
    }

    #[must_use]
    pub fn boarding(due: TripTime, by: ShownTime, now: TripTime) -> TimeCheck {
        Self::early(now.minutes_since(due), by)
    }

    fn early(over: Minutes, by: ShownTime) -> TimeCheck {
        match over.cmp(&Minutes::ZERO) {
            Ordering::Less | Ordering::Equal => TimeCheck::Expected { by, left: -over },
            Ordering::Greater => TimeCheck::Due { by, over },
        }
    }

    #[must_use]
    pub fn relocated(self, off_route: Option<Distance>, from: Option<FactTime>) -> TimeCheck {
        match off_route {
            Some(_) => self,
            None => self.estimated(from),
        }
    }

    #[must_use]
    pub fn estimated(self, from: Option<FactTime>) -> TimeCheck {
        match self {
            TimeCheck::Expected { by, left } => TimeCheck::Estimated { by, left, from },
            TimeCheck::Estimated { .. } | TimeCheck::Due { .. } | TimeCheck::Late { .. } => self,
        }
    }

    #[must_use]
    pub fn expected_by(self) -> ShownTime {
        match self {
            TimeCheck::Expected { by, .. }
            | TimeCheck::Estimated { by, .. }
            | TimeCheck::Due { by, .. }
            | TimeCheck::Late { by, .. } => by,
        }
    }

    #[must_use]
    pub fn rest(self) -> Spelling {
        match self {
            TimeCheck::Expected { left, .. }
            | TimeCheck::Estimated {
                left, from: None, ..
            } => Spelling::new(format!(" · {left} left")),
            TimeCheck::Estimated {
                left,
                from: Some(from),
                ..
            } => Spelling::new(format!(" · {left} left (from {from})")),
            TimeCheck::Due { .. } => Spelling::new(" · now".to_owned()),
            TimeCheck::Late { .. } => Spelling::new(String::new()),
        }
    }

    #[must_use]
    pub fn late_by(self) -> Option<Minutes> {
        match self {
            TimeCheck::Late { over, .. } => Some(over),
            TimeCheck::Expected { .. } | TimeCheck::Estimated { .. } | TimeCheck::Due { .. } => {
                None
            }
        }
    }

    #[must_use]
    pub fn emphasis(self) -> Emphasis {
        match self {
            TimeCheck::Late { .. } => Emphasis::Inverted,
            TimeCheck::Expected { .. } | TimeCheck::Estimated { .. } | TimeCheck::Due { .. } => {
                Emphasis::Plain
            }
        }
    }
}

impl fmt::Display for TimeCheck {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.late_by()
            .map_or(Ok(()), |over| write!(formatter, "LATE {over} · "))?;
        write!(formatter, "by {}{}", self.expected_by(), self.rest())
    }
}
