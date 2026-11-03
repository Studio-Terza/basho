use std::fmt;

use guide::geo::Distance;
use guide::guidance::{Guidance, LegProgress};

use crate::app::session::tracker_decision::TrackerDecision;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackerReading {
    decision: TrackerDecision,
    along: Distance,
    off_route: Option<Distance>,
}

impl TrackerReading {
    #[must_use]
    pub fn seen_in(guidance: &Guidance<'_>, progress: LegProgress) -> TrackerReading {
        TrackerReading {
            decision: TrackerDecision::seen_in(guidance, progress),
            along: guidance.along(),
            off_route: guidance.off_route(),
        }
    }
}

impl fmt::Display for TrackerReading {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}, {} along", self.decision, self.along)?;
        self.off_route
            .map_or(Ok(()), |off| write!(formatter, ", {off} off the route"))
    }
}
