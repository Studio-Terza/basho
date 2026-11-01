use serde::Deserialize;

use crate::geo::Speed;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gait {
    #[default]
    Walk,
    Jog,
}

impl Gait {
    const WALK_FASTEST: Speed = Speed::kilometers_per_hour(5.5);
    const JOG_FASTEST: Speed = Speed::kilometers_per_hour(8.0);

    #[must_use]
    pub fn fastest(self) -> Speed {
        match self {
            Gait::Walk => Self::WALK_FASTEST,
            Gait::Jog => Self::JOG_FASTEST,
        }
    }
}
