use std::collections::BTreeSet;

use crate::NodeId;
use crate::itinerary::Problem;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link(BTreeSet<NodeId>);

impl Link {
    #[must_use]
    pub fn between(from: NodeId, to: NodeId) -> Link {
        Link(BTreeSet::from([from, to]))
    }
}

impl Link {
    pub fn shared(edge: Option<&Link>) -> Result<(), Problem> {
        match edge {
            Some(_) => Ok(()),
            None => Err(Problem::SegmentsApart),
        }
    }
}
