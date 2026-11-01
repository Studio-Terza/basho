use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use newtype::Newtype;

use crate::app::session::Visit;

#[derive(Newtype)]
pub struct ScrollAnchor {
    visit: Visit,
}

impl IntoDynNode for ScrollAnchor {
    fn into_dyn_node(self) -> DynamicNode {
        rsx! {
            {std::iter::once(self.visit).map(|visit| rsx! {
                div { key: "{visit}", class: "scroll-anchor", "aria-hidden": "true",
                    onmounted: move |event| {
                        let mounted = event.data();
                        async move {
                            mounted
                                .scroll_to_with_options(ScrollToOptions {
                                    behavior: ScrollBehavior::Instant,
                                    vertical: ScrollLogicalPosition::Start,
                                    horizontal: ScrollLogicalPosition::Nearest,
                                })
                                .await
                                .unwrap_or_default();
                        }
                    },
                }
            })}
        }
        .into_dyn_node()
    }
}
