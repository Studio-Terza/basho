use serde::Deserialize;

use crate::geo::{Coordinate, Degrees};
use crate::way::{NoMappedPath, WalkSegment};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnmappedStretch {
    via: Vec<[Degrees; Coordinate::LON_LAT_LENGTH]>,
}

impl UnmappedStretch {
    #[must_use]
    pub fn segment(&self) -> WalkSegment {
        WalkSegment::NoMappedPath(NoMappedPath::spanning(
            self.via
                .iter()
                .map(|[lon, lat]| Coordinate::new(*lat, *lon))
                .collect(),
        ))
    }
}
