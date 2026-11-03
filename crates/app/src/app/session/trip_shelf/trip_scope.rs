use dioxus::dioxus_core::{DynamicNode, VComponent};
use dioxus::prelude::*;

use crate::app::session::trip_shelf::TripShelf;
use crate::app::session::trip_shelf::trip_round::TripRound;

#[derive(Props, Clone, PartialEq)]
pub struct TripScope {
    round: TripRound,
    shelf: TripShelf,
}

impl TripScope {
    #[must_use]
    pub fn new(round: TripRound, shelf: TripShelf) -> TripScope {
        TripScope { round, shelf }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "dioxus hands component props by value"
    )]
    pub fn render(scope: TripScope) -> Element {
        scope
            .shelf
            .trip()
            .zip(scope.shelf.page())
            .map_or_else(VNode::empty, |(trip, page)| page(&trip, scope.shelf))
    }
}

impl IntoDynNode for TripScope {
    fn into_dyn_node(self) -> DynamicNode {
        DynamicNode::Component(VComponent::new(Self::render, self, "TripScope"))
    }
}
