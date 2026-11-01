use std::collections::{BTreeMap, BTreeSet};

use crate::geo::{Coordinate, RouteLine};
use crate::way::{MappedWay, NodeId};

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

    #[must_use]
    pub fn joins(&self, from: NodeId, to: NodeId) -> bool {
        self.links
            .get(&from)
            .is_some_and(|linked| linked.contains(&to))
    }
}
