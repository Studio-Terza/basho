use map_plan::{KeyEntry, PickedMark, PinchWay};
use serde::Deserialize;

use crate::app::session::Session;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EngineEvent {
    Shown(Vec<KeyEntry>),
    Picked(Option<PickedMark>),
    Pinch(PinchWay),
    Failed,
}

impl EngineEvent {
    pub fn applied(self, session: &Session) {
        match self {
            EngineEvent::Shown(shown) => session.show_key(shown),
            EngineEvent::Picked(mark) => session.pick_mark(mark),
            EngineEvent::Pinch(way) => session.pinch_map(way),
            EngineEvent::Failed => session.note_map_failed(),
        }
    }
}
