use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;

use crate::app::session::trip_shelf::{PickedFile, TripShelf};

impl IntoDynNode for TripShelf {
    fn into_dyn_node(self) -> DynamicNode {
        let holder = self;
        let refusal = holder.refusal();
        rsx! {
            section { class: "trip-picker",
                {refusal.map(|refusal| rsx! {
                    p { class: "trip-refusal", role: "alert", "{refusal}" }
                })}
                label { class: "place-link trip-open", "Open a trip file"
                    input { class: "trip-file", r#type: "file", accept: ".toml,text/plain", "aria-label": "Open a trip file" }
                }
                button { class: "place-link trip-built-in",
                    onclick: move |_| holder.offer(&PickedFile::built_in()),
                    "Use the built-in trip"
                }
                p { class: "tagline", "Trip files stay on this phone. A new trip clears your progress in the current trip." }
            }
        }
        .into_dyn_node()
    }
}
