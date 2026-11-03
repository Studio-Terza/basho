mod fix_text;
mod screen_text;
mod step_text;
mod tracker_reading;

use dioxus::prelude::*;
use guide::clock::TripTime;
use guide::clock::trip_stamp::TripStamp;
use report::{Capture, Fact, ReportText};

use crate::app::session::{Session, SessionEvent};
use crate::app::step::{DayPoint, Spot, Step};
use crate::app::visit::Screen;
use fix_text::FixText;
use screen_text::ScreenText;
use step_text::StepText;
use tracker_reading::TrackerReading;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportEvent {
    Capture,
    Export,
    Show,
}

impl ReportEvent {
    pub fn reported(self, session: &Session) {
        let mut reports = session.reports;
        match self {
            ReportEvent::Capture => {
                let now = session.catch_up_clock().unwrap_or_else(|| session.now());
                let captured = reports
                    .peek()
                    .clone()
                    .captured(Self::capture_at(session, now));
                reports.set(captured);
                SessionEvent::CloseMenu.applied(session);
                SessionEvent::OpenTab {
                    screen: Screen::Reports,
                    top: *session.touch_top.peek(),
                }
                .applied(session);
            }
            ReportEvent::Export => {
                let exported = reports.peek().clone().exported();
                reports.set(exported);
            }
            ReportEvent::Show => {
                SessionEvent::CloseMenu.applied(session);
                SessionEvent::OpenTab {
                    screen: Screen::Reports,
                    top: *session.touch_top.peek(),
                }
                .applied(session);
            }
        }
    }

    fn capture_at(session: &Session, now: TripTime) -> Capture {
        let today = session.day_at(now);
        let tracker = today
            .map(|day| TrackerReading::seen_in(&session.guidance(day), session.progress()))
            .map(|reading| ReportText::rendered(&reading));
        let fix = (*session.position.peek())
            .map(|sighting| FixText::new(sighting, *session.spread.peek()))
            .map(|fix| ReportText::rendered(&fix));
        Capture::new(ReportText::rendered(&TripStamp::new(now)))
            .knowing(
                Fact::Screen,
                Some(ReportText::rendered(&Self::screen_text(session))),
            )
            .knowing(Fact::Step, Self::step_text(session, now))
            .knowing(Fact::Position, fix)
            .knowing(Fact::Tracker, tracker)
            .after_decisions(session.decisions.peek().texts_at(now))
    }

    fn screen_text(session: &Session) -> ScreenText {
        let screen = *session.screen.peek();
        ScreenText::new(
            screen,
            screen
                .wiki_entry()
                .and_then(|entry| session.wiki.entry(entry)),
        )
    }

    fn step_text(session: &Session, now: TripTime) -> Option<ReportText> {
        let browsing = session.visit.peek().focus().spot();
        browsing
            .map(StepText::Browsing)
            .or_else(|| Self::current_step(session, now))
            .map(|step| ReportText::rendered(&step))
    }

    fn current_step(session: &Session, now: TripTime) -> Option<StepText> {
        let today = session.day_at(now)?;
        DayPoint::locate(&session.guidance(today), &Step::of_day(today))
            .current()
            .map(|step| StepText::Current(Spot::new(today.number(), step)))
    }
}
