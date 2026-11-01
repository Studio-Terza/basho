mod contact;
mod fingers;
use dioxus::prelude::*;

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

    #[expect(
        clippy::needless_for_each,
        reason = "iterators are driven by combinators here"
    )]
    fn relay(self, contact: Contact, session: &Session, event: &Event<TouchData>) {
        contact
            .top(event)
            .into_iter()
            .for_each(|top| SessionEvent::NoteTouchTop(top).applied(session));
        match (
            self,
            contact,
            contact.point(event),
            Fingers::touching(event),
        ) {
            (_, Contact::Began, Some(at), Fingers::Pair) => {
                SessionEvent::PairTouched {
                    at,
                    spread: Fingers::spread(event).unwrap_or_default(),
                }
                .applied(session);
            }
            (TouchSurface::Page, Contact::Began, Some(at), Fingers::Single) => {
                SessionEvent::PageTouchBegan(at).applied(session);
            }
            (TouchSurface::Menu, Contact::Began, Some(at), Fingers::Single) => {
                SessionEvent::MenuTouchBegan(at).applied(session);
            }
            (_, Contact::Moved, Some(at), _) => {
                Fingers::spread(event)
                    .into_iter()
                    .for_each(|spread| SessionEvent::PairSpread(spread).applied(session));
                SessionEvent::TouchMoved(at).applied(session);
            }
            (_, Contact::Ended, Some(at), _) => SessionEvent::TouchEnded(at).applied(session),
            (_, Contact::Cancelled, _, _) => SessionEvent::TouchCancelled.applied(session),
            (TouchSurface::Tabs, Contact::Began, Some(_), Fingers::Single)
            | (_, Contact::Began | Contact::Moved | Contact::Ended, None, _) => (),
        }
    }
}
