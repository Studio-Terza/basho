use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::geo::{Coordinate, Degrees, RouteLine};
use crate::itinerary::Problem;
use crate::way::{LegPath, RoadNetwork, SegmentEntry};

#[derive(Debug, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum LegEntry {
    Walked {
        segments: Vec<SegmentEntry>,
    },
    Ridden {
        #[expect(dead_code, reason = "kept so the file loads")]
        km: Option<IgnoredAny>,
        coords: Vec<[Degrees; Coordinate::LON_LAT_LENGTH]>,
    },
}

impl LegEntry {
    pub fn path(&self, network: &RoadNetwork) -> Result<LegPath, Problem> {
        match self {
            LegEntry::Walked { segments } => LegPath::walked(segments, network),
            LegEntry::Ridden { coords, .. } => Ok(LegPath::ridden(RouteLine::new(
                coords
                    .iter()
                    .map(|[lon, lat]| Coordinate::new(*lat, *lon))
                    .collect(),
            ))),
        }
    }
}
