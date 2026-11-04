use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::plane::Halving;
use guide::plane::PlanePoint;
use layout::MarkSize;
use map_plan::KeyEntry;
use map_plan::MarkAt;
use newtype::Newtype;

use crate::app::session::Session;

#[derive(Newtype)]
pub struct MapKeyView<'view> {
    session: &'view Session,
}

impl MapKeyView<'_> {
    fn swatch(entry: KeyEntry) -> Element {
        let middle = MarkSize::Large.half() + MarkSize::outline_reach();
        let side = middle.twice();
        entry.mark().map_or_else(
            || {
                rsx! {
                    svg { class: "key-swatch", view_box: "0 0 24 16", "aria-hidden": "true",
                        path { class: "key-sample {entry.class()}", d: "M1 8h22" }
                    }
                }
            },
            |mark| {
                rsx! {
                    svg { class: "key-swatch", view_box: "0 0 {side.pixels()} {side.pixels()}", "aria-hidden": "true",
                        {MarkAt::new(mark, PlanePoint::new(middle, middle))}
                    }
                }
            },
        )
    }
}

impl IntoDynNode for MapKeyView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let shown = self.session.key_shown();
        rsx! {
            {(!shown.is_empty()).then(|| rsx! {
                details { class: "map-key", "data-overlay": "true",
                    summary { "Key" }
                    ul {
                        {shown.iter().map(|entry| rsx! {
                            li { {Self::swatch(*entry)} span { "{entry.meaning()}" } }
                        })}
                    }
                }
            })}
        }
        .into_dyn_node()
    }
}
