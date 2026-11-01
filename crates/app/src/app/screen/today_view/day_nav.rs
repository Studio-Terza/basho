use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::itinerary::Day;

use crate::app::screen::day_name::DayName;
use crate::app::screen::look::Look;
use crate::app::session::{Landing, Session, SessionEvent};
use crate::app::step::Toward;

pub struct DayNav<'view> {
    session: &'view Session,
    day: &'view Day,
}

impl<'view> DayNav<'view> {
    pub fn new(session: &'view Session, day: &'view Day) -> Self {
        DayNav { session, day }
    }

    fn link(&self, target: &Day, look: Look, label: &str) -> Element {
        let session = self.session.clone();
        let number = target.number();
        rsx! {
            button { class: look.class(), onclick: move |_| session.jump(number, Landing::Top), "{label}" }
        }
    }
}

impl IntoDynNode for DayNav<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let previous = self
            .session
            .neighbour_day(self.day.number(), Toward::Previous);
        let next = self.session.neighbour_day(self.day.number(), Toward::Next);
        let opener = self.session.clone();
        rsx! {
            nav { class: "day-nav",
                button { class: "days-open", "aria-label": "Days", onclick: move |_| SessionEvent::OpenMenu.applied(&opener), "Days" }
                {previous.map(|target| self.link(target, Look::DayBack, &format!("‹ {}", DayName::new(target.number()))))}
                h1 { class: "day-title", "{DayName::new(self.day.number())} · {self.day.date().short()}" }
                {next.map(|target| self.link(target, Look::DayNext, &format!("{} ›", DayName::new(target.number()))))}
            }
        }
        .into_dyn_node()
    }
}
