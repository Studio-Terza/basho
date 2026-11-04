use dioxus::prelude::*;

use crate::app::history::StepFolds;
use crate::app::session::Session;
use crate::app::visit::Landing;
use crate::app::visit::Screen;
use crate::app::visit::Scroll;
use crate::app::visit::Visit;

use super::menu_event::MenuEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollEvent {
    Hold,
    ShowCurrentStep,
    BackToNow,
    ShowFocusInToday,
}

impl ScrollEvent {
    pub fn scrolled(self, session: &Session) {
        match self {
            ScrollEvent::Hold => Self::held(session.visit),
            ScrollEvent::ShowCurrentStep => Self::current_step(session),
            ScrollEvent::BackToNow => Self::back_to_now(session, session.folds),
            ScrollEvent::ShowFocusInToday => Self::focus_in_today(session, session.visit),
        }
    }

    fn held(mut visit: Signal<Visit>) {
        let current = *visit.peek();
        let held = current.scrolled(Scroll::Hold);
        (held != current).then(|| visit.set(held));
    }

    fn current_step(session: &Session) {
        ScrollEvent::BackToNow.scrolled(session);
        session.show(Screen::Today);
    }

    fn back_to_now(session: &Session, mut folds: Signal<StepFolds>) {
        let mut visit = session.visit;
        let current = *visit.peek();
        visit.set(current.home());
        folds.set(StepFolds::default());
        MenuEvent::Close.moved_menu(session);
    }

    fn focus_in_today(session: &Session, mut visit: Signal<Visit>) {
        let current = *visit.peek();
        let landing = current
            .focus()
            .spot()
            .map_or(Landing::Now, |spot| Landing::Step(spot.step()));
        visit.set(current.jumped(current.focus(), landing));
        session.show(Screen::Today);
    }
}
