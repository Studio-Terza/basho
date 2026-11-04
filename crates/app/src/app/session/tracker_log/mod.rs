mod logged_decision;

use guide::clock::TripTime;
use report::ReportText;

use crate::app::session::measure::Cadence;
use crate::app::session::tracker_decision::TrackerDecision;
use logged_decision::LoggedDecision;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DecisionLog {
    decisions: Vec<LoggedDecision>,
}

impl DecisionLog {
    #[must_use]
    pub fn noted(&self, at: TripTime, decision: TrackerDecision) -> Option<DecisionLog> {
        self.decisions
            .last()
            .is_none_or(|last| last.decision() != decision)
            .then(|| DecisionLog {
                decisions: self
                    .within_window_of(at)
                    .chain([LoggedDecision::new(at, decision)])
                    .collect(),
            })
    }

    #[must_use]
    pub fn texts_at(&self, now: TripTime) -> Vec<ReportText> {
        self.within_window_of(now)
            .map(|logged| ReportText::rendered(&logged))
            .collect()
    }

    fn within_window_of(&self, now: TripTime) -> impl Iterator<Item = LoggedDecision> {
        self.decisions.iter().copied().filter(move |logged| {
            logged.logged_at() >= now.earlier_by(Cadence::DecisionMemory.every())
        })
    }
}
