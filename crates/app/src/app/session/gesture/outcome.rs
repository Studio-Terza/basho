use crate::app::session::{Session, SessionEvent};
use crate::app::step::Toward;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Nothing,
    OpenMenu,
    KeepMenu,
    CloseMenu,
    Swipe(Toward),
}

impl Outcome {
    pub fn settle(self, session: &Session) {
        match self {
            Outcome::CloseMenu => SessionEvent::CloseMenu.applied(session),
            Outcome::OpenMenu => SessionEvent::OpenMenu.applied(session),
            Outcome::Swipe(toward) => session.turn_page(toward),
            Outcome::KeepMenu | Outcome::Nothing => (),
        }
    }
}
