use std::collections::BTreeSet;

use crate::NodeId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link(BTreeSet<NodeId>);

impl Link {
    #[must_use]
    pub fn between(from: NodeId, to: NodeId) -> Link {
        Link(BTreeSet::from([from, to]))
    }
}
