use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::plane::PlanePoint;

use super::MapMark;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MarkAt {
    mark: MapMark,
    at: PlanePoint,
}

impl MarkAt {
    #[must_use]
    pub fn new(mark: MapMark, at: PlanePoint) -> MarkAt {
        MarkAt { mark, at }
    }

    #[must_use]
    pub fn mark(self) -> MapMark {
        self.mark
    }

    #[must_use]
    pub fn point(self) -> PlanePoint {
        self.at
    }
}

impl IntoDynNode for MarkAt {
    fn into_dyn_node(self) -> DynamicNode {
        let look = self.mark.look();
        look.shape()
            .draw(self.at, look.size().half(), look)
            .into_dyn_node()
    }
}
