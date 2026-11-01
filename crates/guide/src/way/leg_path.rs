use crate::geo::{Coordinate, RouteLine};
use crate::itinerary::Problem;
use crate::way::{RoadNetwork, SegmentEntry, WalkSegment};

#[derive(derive_more::AsRef, Clone, Debug, Default, PartialEq)]
pub struct LegPath {
    #[as_ref]
    line: RouteLine,
    segments: Vec<WalkSegment>,
}

impl LegPath {
    #[must_use]
    pub fn ridden(line: RouteLine) -> LegPath {
        LegPath {
            line,
            segments: Vec::new(),
        }
    }

    pub fn walked(entries: &[SegmentEntry], network: &RoadNetwork) -> Result<LegPath, Problem> {
        let loose = entries
            .iter()
            .map(|entry| entry.segment(network))
            .collect::<Result<Vec<WalkSegment>, Problem>>()?;
        loose
            .windows(RouteLine::SEGMENT_ENDS_LENGTH)
            .try_for_each(|pair| pair[0].run_into(&pair[1]))?;
        let segments: Vec<WalkSegment> = loose
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, segment)| {
                let before = index.checked_sub(1).and_then(|earlier| loose.get(earlier));
                segment.between(before, loose.get(index + 1))
            })
            .collect();
        let mut points: Vec<Coordinate> = segments
            .iter()
            .flat_map(|segment| AsRef::<RouteLine>::as_ref(segment).points().iter().copied())
            .collect();
        points.dedup();
        Ok(LegPath {
            line: RouteLine::new(points),
            segments,
        })
    }

    #[must_use]
    pub fn segments(&self) -> &[WalkSegment] {
        &self.segments
    }
}
