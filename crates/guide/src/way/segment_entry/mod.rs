mod mapped_run;
mod unmapped_stretch;

use serde::Deserialize;

use crate::itinerary::Problem;
use crate::way::{RoadNetwork, WalkSegment};
use mapped_run::MappedRun;
use unmapped_stretch::UnmappedStretch;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum SegmentEntry {
    Mapped(MappedRun),
    NoMappedPath(UnmappedStretch),
}

impl SegmentEntry {
    pub fn segment(&self, network: &RoadNetwork) -> Result<WalkSegment, Problem> {
        match self {
            SegmentEntry::Mapped(run) => run.segment(network).map(WalkSegment::Mapped),
            SegmentEntry::NoMappedPath(stretch) => Ok(stretch.segment()),
        }
    }
}
