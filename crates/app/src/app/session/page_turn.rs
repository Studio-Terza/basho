use dioxus::prelude::*;
use guide::day_number::DayNumber;

use crate::app::session::{Landing, Session, SessionEvent};
use crate::app::step::{Spot, Toward};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageTurn {
    Step(Spot),
    Day(DayNumber),
    History(Toward),
}

impl PageTurn {
    pub fn show(self, session: &Session) {
        match self {
            PageTurn::Step(spot) => session.browse(spot),
            PageTurn::Day(day) => session.jump(day, Landing::Top),
            PageTurn::History(toward) => SessionEvent::TurnHistory {
                toward,
                top: *session.touch_top.peek(),
            }
            .applied(session),
        }
    }
}
