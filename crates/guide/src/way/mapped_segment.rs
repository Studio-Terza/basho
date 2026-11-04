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
    pub fn meets<'next>(&self, next: &'next MappedSegment) -> Option<&'next MappedSegment> {
        Some(next)
            .filter(|next| self.line.points().last() == next.line.points().first())
            .filter(|next| {
                Some(*next)
                    .filter(|next| self.nodes.last() == next.nodes.first())
                    .or(Some(*next).filter(|next| self.edge(End::Finish) == next.edge(End::Start)))
                    .is_some()
            })
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
