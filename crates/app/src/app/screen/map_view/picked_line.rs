use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use newtype::Newtype;

use crate::app::session::Session;

#[derive(Newtype)]
pub struct PickedLine<'view> {
    session: &'view Session,
}

impl IntoDynNode for PickedLine<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        rsx! {
            {self.session.picked().map(|picked| rsx! {
                p { class: "map-picked", "{picked}" }
            })}
        }
        .into_dyn_node()
    }
}
