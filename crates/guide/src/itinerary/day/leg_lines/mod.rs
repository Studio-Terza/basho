mod leg_entry;
use std::collections::BTreeMap;

use serde::Deserialize;

use crate::day_number::DayNumber;
use crate::itinerary::Day;
use crate::itinerary::Problem;
use crate::leg::LegIndex;
use crate::way::{LegPath, MappedWay, RoadNetwork};
use leg_entry::LegEntry;

#[derive(derive_more::AsRef, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegLines {
    #[serde(default)]
    #[as_ref]
    day: DayNumber,
    #[serde(default)]
    ways: Vec<MappedWay>,
    legs: BTreeMap<LegIndex, LegEntry>,
}

impl LegLines {
    pub fn beyond<'day>(&'day self, day: &'day Day) -> impl Iterator<Item = LegIndex> + 'day {
        self.legs
            .keys()
            .copied()
            .filter(|leg| day.leg(*leg).is_none())
    }

    pub fn path(&self, leg: LegIndex, network: &RoadNetwork) -> Result<LegPath, Problem> {
        self.legs
            .get(&leg)
            .map_or_else(|| Ok(LegPath::default()), |entry| entry.path(network))
    }
}

impl From<&LegLines> for RoadNetwork {
    fn from(lines: &LegLines) -> RoadNetwork {
        RoadNetwork::new(&lines.ways)
    }
}
