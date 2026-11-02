use dioxus::prelude::*;
use guide::clock::TripTime;
use guide::guidance::LegProgress;
use guide::guidance::tracking::Placing;
use guide::itinerary::Day;

use crate::app::session::Session;
use crate::app::session::progress_file::ProgressFile;
use crate::app::step::{Spot, Step};

use super::scroll_event::ScrollEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinEvent {
    Here(Spot),
    Done(Spot),
}

impl PinEvent {
    fn spot(self) -> Spot {
        match self {
            PinEvent::Here(spot) | PinEvent::Done(spot) => spot,
        }
    }

    fn placing_in(self, steps: &[Step<'_>]) -> Option<Placing> {
        steps
            .iter()
            .find(|step| step.number() == self.spot().step())
            .map(|step| match self {
                PinEvent::Here(_) => step.placing(),
                PinEvent::Done(_) => step.finished_in(steps),
            })
    }

    fn pin_in(self, session: &Session, day: &Day, now: TripTime) {
        let mut progress = session.progress;
        let baseline = session.guidance(day).placing();
        self.placing_in(&Step::of_day(day))
            .into_iter()
            .for_each(|placing| progress.set(LegProgress::pinned_at(day, placing, now, baseline)));
    }

    pub fn pinned(self, session: &Session) {
        let now = session.catch_up_clock().unwrap_or_else(|| session.now());
        session
            .day_at(now)
            .filter(|day| day.number() == self.spot().day())
            .into_iter()
            .for_each(|day| self.pin_in(session, day, now));
        ProgressFile::keep_in(
            session.shelf().home().as_ref(),
            *session.progress.peek(),
            session.itinerary(),
        );
        ScrollEvent::BackToNow.scrolled(session);
    }
}
