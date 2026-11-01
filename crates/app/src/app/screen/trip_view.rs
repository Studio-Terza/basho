use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::english_name::EnglishName;

pub struct TripView<'name, Back, Shelf, Dark> {
    name: &'name EnglishName,
    back: Back,
    shelf: Shelf,
    dark: Dark,
}

impl<'name, Back: Fn(Event<MouseData>) + 'static, Shelf: IntoDynNode, Dark: IntoDynNode>
    TripView<'name, Back, Shelf, Dark>
{
    pub fn new(name: &'name EnglishName, back: Back, shelf: Shelf, dark: Dark) -> Self {
        TripView {
            name,
            back,
            shelf,
            dark,
        }
    }
}

impl<Back: Fn(Event<MouseData>) + 'static, Shelf: IntoDynNode, Dark: IntoDynNode> IntoDynNode
    for TripView<'_, Back, Shelf, Dark>
{
    fn into_dyn_node(self) -> DynamicNode {
        let name = self.name;
        let back = self.back;
        let picker = self.shelf;
        let dark = self.dark;
        rsx! {
            article { class: "trip-view",
                button { class: "day-back place-back", onclick: back, "‹ Back" }
                header {
                    h1 { "{name}" }
                    p { class: "tagline", "The trip on this phone" }
                }
                {picker}
                {dark}
            }
        }
        .into_dyn_node()
    }
}
