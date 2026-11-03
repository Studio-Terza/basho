use std::time::Instant;

use dioxus::prelude::*;
use guide::plane::Axis;
use guide::plane::Pixels;
use guide::plane::PlanePoint;
use menu::MenuStep;

use crate::app::session::gesture::{Gesture, Origin, TouchSample};
use crate::app::session::measure::Cadence;
use crate::app::session::{Session, SessionEvent};
use crate::app::visit::Screen;

use super::scroll_event::ScrollEvent;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TouchEvent {
    PageBegan(PlanePoint),
    MenuBegan(PlanePoint),
    PairBegan { at: PlanePoint, spread: Pixels },
    PairSpread(Pixels),
    PairHeld(Gesture),
    Moved(PlanePoint),
    Ended(PlanePoint),
    Cancelled,
}

impl TouchEvent {
    pub fn touched(self, session: &Session) {
        let mut gesture = session.gesture;
        let mut pull = session.pull;
        match self {
            TouchEvent::PageBegan(at) => gesture.set(Gesture::pressed(
                Origin::on_page(at[Axis::Across]),
                TouchSample::new(at, Instant::now()),
            )),
            TouchEvent::MenuBegan(at) => gesture.set(Gesture::pressed(
                Origin::Menu,
                TouchSample::new(at, Instant::now()),
            )),
            TouchEvent::PairBegan { at, spread } => {
                TouchEvent::Cancelled.touched(session);
                let paired = Gesture::paired(TouchSample::new(at, Instant::now()), spread);
                gesture.set(paired);
                let holder = session.clone();
                spawn(async move {
                    tokio::time::sleep(Cadence::PairHold.every()).await;
                    SessionEvent::PairHeld(paired).applied(&holder);
                });
            }
            TouchEvent::PairSpread(now) => {
                let spreading = gesture.peek().spread_to(now);
                gesture.set(spreading);
            }
            TouchEvent::PairHeld(paired) => {
                let current = *gesture.peek();
                (current == paired).then(|| {
                    gesture.set(Gesture::Idle);
                    SessionEvent::CaptureReport.applied(session);
                });
            }
            TouchEvent::Moved(at) => {
                let mut menu = session.menu;
                let moved = gesture.peek().moved(TouchSample::new(at, Instant::now()));
                gesture.set(moved);
                (moved == Gesture::Scrolling && *session.screen.peek() == Screen::Today)
                    .then(|| ScrollEvent::Hold.scrolled(session));
                let pulled = moved.pull(session.viewport());
                let previous = *pull.peek();
                (previous != pulled).then(|| pull.set(pulled));
                let current = *menu.peek();
                let dragged = pulled
                    .filter(|_| moved.origin() == Some(Origin::Edge))
                    .map_or(current, |_| current.after(MenuStep::Open));
                (dragged != current).then(|| menu.set(dragged));
                session.lean_toward(
                    moved
                        .swipe(session.viewport())
                        .filter(|toward| session.page_turn(*toward).is_some()),
                );
            }
            TouchEvent::Ended(at) => {
                let outcome = gesture
                    .peek()
                    .outcome(TouchSample::new(at, Instant::now()), session.viewport());
                gesture.set(Gesture::Idle);
                pull.set(None);
                session.lean_toward(None);
                outcome.settle(session);
            }
            TouchEvent::Cancelled => {
                let outcome = gesture.peek().abandoned();
                gesture.set(Gesture::Idle);
                pull.set(None);
                session.lean_toward(None);
                outcome.settle(session);
            }
        }
    }
}
