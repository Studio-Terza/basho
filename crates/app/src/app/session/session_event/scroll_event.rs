use dioxus::prelude::*;

use crate::app::history::StepFolds;
use crate::app::screen::Screen;
use crate::app::session::visit::Scroll;
use crate::app::session::{Landing, Session};

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
        let mut visit = session.visit;
        let current = *visit.peek();
        match self {
            ScrollEvent::Hold => {
                let held = current.scrolled(Scroll::Hold);
                (held != current).then(|| visit.set(held));
            }
            ScrollEvent::ShowCurrentStep => {
                ScrollEvent::BackToNow.scrolled(session);
                session.show(Screen::Today);
            }
            ScrollEvent::BackToNow => {
                visit.set(current.home());
                let mut folds = session.folds;
                folds.set(StepFolds::default());
                MenuEvent::Close.moved_menu(session);
            }
            ScrollEvent::ShowFocusInToday => {
                let landing = current
                    .focus()
                    .spot()
                    .map_or(Landing::Now, |spot| Landing::Step(spot.step()));
                visit.set(current.jumped(current.focus(), landing));
                session.show(Screen::Today);
            }
        }
    }
}
