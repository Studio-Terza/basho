use std::fmt;

use guide::clock::TripTime;
use guide::clock::trip_stamp::TripStamp;

use crate::app::session::tracker_decision::TrackerDecision;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoggedDecision {
    at: TripTime,
    decision: TrackerDecision,
}

impl LoggedDecision {
    #[must_use]
    pub fn new(at: TripTime, decision: TrackerDecision) -> LoggedDecision {
        LoggedDecision { at, decision }
    }

    #[must_use]
    pub fn decided(self, decision: TrackerDecision) -> bool {
        self.decision == decision
    }

    #[must_use]
    pub fn since(self, earliest: TripTime) -> bool {
        self.at >= earliest
    }
}

impl fmt::Display for LoggedDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", TripStamp::new(self.at), self.decision)
    }
}
