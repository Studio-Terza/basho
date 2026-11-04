use super::link::Link;
use crate::NodeId;
use crate::end::End;
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

    pub fn meets(&self, next: &MappedSegment) -> Result<Join, Problem> {
        match self.touching(next) {
            Some(_) => Join::new(self.shared_node(next), self.shared_edge(next)),
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
        self.edge(End::Finish)
            .filter(|edge| Some(*edge) == next.edge(End::Start))
    }

    fn edge(&self, end: End) -> Option<&Link> {
        match end {
            End::Start => self.links.first(),
            End::Finish => self.links.last(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Join {
    ByNode,
    ByEdge,
}

impl Join {
    fn new(node: Option<&NodeId>, edge: Option<&Link>) -> Result<Join, Problem> {
        match node {
            Some(_) => Ok(Join::ByNode),
            None => Join::by_edge(edge),
        }
    }

    fn by_edge(edge: Option<&Link>) -> Result<Join, Problem> {
        match edge {
            Some(_) => Ok(Join::ByEdge),
            None => Err(Problem::SegmentsApart),
        }
    }
}
