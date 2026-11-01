use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::evidence::Evidence;
use newtype::Newtype;

#[derive(Newtype)]
pub struct UnverifiedMark<'evidence> {
    evidence: &'evidence Evidence,
}

impl IntoDynNode for UnverifiedMark<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        self.evidence.mark().map_or(DynamicNode::default(), |mark| {
            rsx! {
                sup { class: "unverified", title: "Not checked against a source", "{mark}" }
            }
            .into_dyn_node()
        })
    }
}
