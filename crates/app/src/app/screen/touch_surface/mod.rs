mod contact;
mod fingers;
use dioxus::prelude::*;
use guide::plane::PlanePoint;

use crate::app::session::{Session, SessionEvent};
use contact::Contact;
use fingers::Fingers;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TouchSurface {
    Page,
    Menu,
    Tabs,
}

impl TouchSurface {
    pub fn wrap(self, session: &Session, body: Element) -> Element {
        let [start, moving, end, cancel] = std::array::from_fn(|_| session.clone());
        rsx! {
            div { class: "touch-surface",
                ontouchstart: move |event| self.relay(Contact::Began, &start, &event),
                ontouchmove: move |event| self.relay(Contact::Moved, &moving, &event),
                ontouchend: move |event| self.relay(Contact::Ended, &end, &event),
                ontouchcancel: move |event| self.relay(Contact::Cancelled, &cancel, &event),
                {body}
            }
        }
    }

    pub fn tabs(session: &Session, buttons: Element) -> Element {
        let [start, moving, end, cancel] = std::array::from_fn(|_| session.clone());
        rsx! {
            nav { class: "tabs",
                ontouchstart: move |event| Self::Tabs.relay(Contact::Began, &start, &event),
                ontouchmove: move |event| Self::Tabs.relay(Contact::Moved, &moving, &event),
                ontouchend: move |event| Self::Tabs.relay(Contact::Ended, &end, &event),
                ontouchcancel: move |event| Self::Tabs.relay(Contact::Cancelled, &cancel, &event),
                {buttons}
            }
        }
    }

    fn relay(self, contact: Contact, session: &Session, event: &Event<TouchData>) {
        contact
            .top(event)
            .map(SessionEvent::NoteTouchTop)
            .into_iter()
            .chain(self.touched(contact, event))
            .for_each(|touch| touch.applied(session));
    }

    fn touched(self, contact: Contact, event: &Event<TouchData>) -> Vec<SessionEvent> {
        match contact {
            Contact::Began => self
                .began(Fingers::touching(event), contact.point(event), event)
                .into_iter()
                .collect(),
            Contact::Moved => contact
                .point(event)
                .into_iter()
                .flat_map(|at| {
                    Fingers::spread(event)
                        .map(SessionEvent::PairSpread)
                        .into_iter()
                        .chain([SessionEvent::TouchMoved(at)])
                })
                .collect(),
            Contact::Ended => contact
                .point(event)
                .map(SessionEvent::TouchEnded)
                .into_iter()
                .collect(),
            Contact::Cancelled => vec![SessionEvent::TouchCancelled],
        }
    }

    fn began(
        self,
        fingers: Fingers,
        at: Option<PlanePoint>,
        event: &Event<TouchData>,
    ) -> Option<SessionEvent> {
        match fingers {
            Fingers::Pair => at.map(|at| SessionEvent::PairTouched {
                at,
                spread: Fingers::spread(event).unwrap_or_default(),
            }),
            Fingers::Single => at.and_then(|at| self.single(at)),
        }
    }

    fn single(self, at: PlanePoint) -> Option<SessionEvent> {
        match self {
            TouchSurface::Page => Some(SessionEvent::PageTouchBegan(at)),
            TouchSurface::Menu => Some(SessionEvent::MenuTouchBegan(at)),
            TouchSurface::Tabs => None,
        }
    }
}
