use dioxus::prelude::*;
use guide::guidance::{Guidance, Sighting};
use guide::timeline::Halt;

use crate::app::session::Session;
use crate::app::session::tracker_decision::TrackerDecision;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressEvent {
    TrackWalk,
    Confirm,
    NoteDecision,
}

impl ProgressEvent {
    pub fn progressed(self, session: &Session) {
        match self {
            ProgressEvent::TrackWalk => {
                let mut trail = session.trail;
                let mut halt = session.halt;
                let sighting = *session.position.read();
                let before = trail.peek().clone();
                let grown = sighting.map_or_else(
                    || before.clone(),
                    |sighting| sighting.recorded_on(before.clone()),
                );
                (grown != before).then(|| trail.set(grown));
                let previous = *halt.peek();
                let tracked = sighting
                    .and_then(Sighting::fresh)
                    .map(|at| Halt::tracked(previous, at, session.clock.time()))
                    .or(previous);
                (tracked != previous).then(|| halt.set(tracked));
            }
            ProgressEvent::Confirm => {
                let mut progress = session.progress;
                let previous = *progress.peek();
                let fresh = session.position.read().and_then(Sighting::fresh);
                let now = session.catch_up_clock().unwrap_or_else(|| session.now());
                session
                    .day_at(now)
                    .map(|day| Guidance::new(day, previous, now, fresh).confirmed(previous))
                    .filter(|confirmed| *confirmed != previous)
                    .into_iter()
                    .for_each(|confirmed| progress.set(confirmed));
            }
            ProgressEvent::NoteDecision => {
                let mut decisions = session.decisions;
                let now = session.now();
                session
                    .day_at(now)
                    .map(|day| TrackerDecision::seen_in(&session.guidance(day), session.progress()))
                    .and_then(|decision| decisions.peek().noted(now, decision))
                    .into_iter()
                    .for_each(|noted| decisions.set(noted));
            }
        }
    }
}
