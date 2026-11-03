use serde::Deserialize;

use crate::clock::ClockTime;
use crate::pressing::Pressing;
use crate::times::ListedTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Cue {
    After(ListedTime),
    Until(ListedTime),
    Departed,
}

impl Cue {
    #[must_use]
    pub fn pressing(self, today: Option<ClockTime>, departed: Pressing) -> Pressing {
        match self {
            Cue::After(time) => today
                .filter(|clock| *clock > ClockTime::from(time))
                .map_or(Pressing::Later, |_| Pressing::Now),
            Cue::Until(time) => today
                .filter(|clock| *clock <= ClockTime::from(time))
                .map_or(Pressing::Later, |_| Pressing::Now),
            Cue::Departed => departed,
        }
    }
}
