use super::link::Link;
use crate::NodeId;
use crate::geo::{Coordinate, RouteLine};
use crate::itinerary::Problem;

#[derive(derive_more::AsRef, Clone, Debug, PartialEq)]
pub struct MappedSegment {
    #[as_ref]
    line: RouteLine,
    nodes: Vec<NodeId>,
    links: Vec<Link>,
}

impl MappedSegment {
    pub(super) fn along(
        points: Vec<Coordinate>,
        nodes: Vec<NodeId>,
        links: Vec<Link>,
    ) -> MappedSegment {
        MappedSegment {
            line: RouteLine::new(points),
            nodes,
            links,
        }
    }

    pub fn meets(&self, next: &MappedSegment) -> Result<(), Problem> {
        match self.touching(next) {
            Some(_) => Self::joined(self.shared_node(next), self.shared_edge(next)),
            None => Err(Problem::SegmentsApart),
        }
    }

    fn touching(&self, next: &MappedSegment) -> Option<&Coordinate> {
        self.line
            .points()
            .last()
            .filter(|end| Some(*end) == next.line.points().first())
    }

    fn shared_node(&self, next: &MappedSegment) -> Option<&NodeId> {
        self.nodes
            .last()
            .filter(|node| Some(*node) == next.nodes.first())
    }

    fn shared_edge(&self, next: &MappedSegment) -> Option<&Link> {
        self.links
            .last()
            .filter(|edge| Some(*edge) == next.links.first())
    }

    fn joined(node: Option<&NodeId>, edge: Option<&Link>) -> Result<(), Problem> {
        match node {
            Some(_) => Ok(()),
            None => Link::shared(edge),
        }
    }
}
