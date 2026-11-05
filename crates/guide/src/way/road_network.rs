use std::collections::{BTreeMap, BTreeSet};

use crate::NodeId;
use crate::geo::{Coordinate, RouteLine};
use crate::itinerary::Problem;
use crate::way::MappedWay;
use crate::way::link::Link;

#[derive(Debug, Default)]
pub struct RoadNetwork {
    places: BTreeMap<NodeId, Coordinate>,
    links: BTreeMap<NodeId, BTreeSet<NodeId>>,
}

impl RoadNetwork {
    #[must_use]
    pub fn new(ways: &[MappedWay]) -> RoadNetwork {
        RoadNetwork {
            places: ways
                .iter()
                .flat_map(|way| way.nodes().iter().copied())
                .fold(BTreeMap::new(), |places, node| node.placed_in(places)),
            links: ways
                .iter()
                .flat_map(|way| way.nodes().windows(RouteLine::SEGMENT_ENDS_LENGTH))
                .fold(BTreeMap::new(), |links, pair| {
                    pair[0].linked_to(pair[1], links)
                }),
        }
    }

    #[must_use]
    pub fn place(&self, node: NodeId) -> Option<Coordinate> {
        self.places.get(&node).copied()
    }

    pub fn link(&self, from: NodeId, to: NodeId) -> Result<Link, Problem> {
        self.links
            .get(&from)
            .and_then(|linked| linked.get(&to))
            .map(|joined| Link::between(from, *joined))
            .ok_or(Problem::NoConnection { from, to })
    }
}
