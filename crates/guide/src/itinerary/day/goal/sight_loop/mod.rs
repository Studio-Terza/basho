mod loop_number;
mod loop_sight;
mod loop_stop;

use serde::Deserialize;

use crate::content::Content;
use crate::words::StarName;

pub use loop_number::LoopNumber;
use loop_sight::LoopSight;
pub use loop_stop::LoopStop;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SightLoop {
    sights: Vec<LoopSight>,
    #[serde(default)]
    stars: Vec<StarName>,
}

impl SightLoop {
    pub fn names(&self) -> impl Iterator<Item = &Content> {
        self.sights.iter().map(AsRef::as_ref)
    }
}

impl AsRef<[StarName]> for SightLoop {
    fn as_ref(&self) -> &[StarName] {
        &self.stars
    }
}

impl<'sight> From<&'sight SightLoop> for Vec<LoopStop<'sight>> {
    fn from(sight_loop: &'sight SightLoop) -> Vec<LoopStop<'sight>> {
        sight_loop
            .sights
            .iter()
            .enumerate()
            .map(|(index, sight)| LoopStop::new(LoopNumber::after(index), sight))
            .collect()
    }
}
