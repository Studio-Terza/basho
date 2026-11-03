use serde::Deserialize;

use std::collections::{BTreeMap, BTreeSet};

use crate::NodeId;
use crate::geo::{Coordinate, Degrees};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct WayNode(NodeId, Degrees, Degrees);

impl WayNode {
    #[must_use]
    pub fn placed_in(
        self,
        mut places: BTreeMap<NodeId, Coordinate>,
    ) -> BTreeMap<NodeId, Coordinate> {
        places.insert(self.0, Coordinate::new(self.2, self.1));
        places
    }

    #[must_use]
    pub fn linked_to(
        self,
        next: WayNode,
        mut links: BTreeMap<NodeId, BTreeSet<NodeId>>,
    ) -> BTreeMap<NodeId, BTreeSet<NodeId>> {
        links.entry(self.0).or_default().insert(next.0);
        links.entry(next.0).or_default().insert(self.0);
        links
    }
}
