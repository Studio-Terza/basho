use std::collections::BTreeSet;

use crate::NodeId;
use crate::end::End;
use crate::geo::{Coordinate, RouteLine};

#[derive(derive_more::AsRef, Clone, Debug, PartialEq)]
pub struct MappedSegment {
    #[as_ref]
    line: RouteLine,
    nodes: Vec<NodeId>,
}

impl MappedSegment {
    pub(super) fn along(points: Vec<Coordinate>, nodes: Vec<NodeId>) -> MappedSegment {
        MappedSegment {
            line: RouteLine::new(points),
            nodes,
        }
    }

    #[must_use]
    pub fn meets(&self, next: &MappedSegment) -> bool {
        self.line.points().last() == next.line.points().first()
            && (self.nodes.last() == next.nodes.first()
                || self.edge(End::Finish) == next.edge(End::Start))
    }

    fn edge(&self, end: End) -> Option<BTreeSet<NodeId>> {
        match end {
            End::Start => self
                .nodes
                .first_chunk::<{ RouteLine::SEGMENT_ENDS_LENGTH }>()
                .map(|ends| ends.iter().copied().collect()),
            End::Finish => self
                .nodes
                .last_chunk::<{ RouteLine::SEGMENT_ENDS_LENGTH }>()
                .map(|ends| ends.iter().copied().collect()),
        }
    }
}
