use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use newtype::Newtype;

use crate::app::session::{Session, SessionEvent};

#[derive(Newtype)]
pub struct DarkSwitch<'session> {
    session: &'session Session,
}

impl IntoDynNode for DarkSwitch<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let session = self.session.clone();
        let setting = self.session.dark_after_sunset();
        rsx! {
            section { class: "dark-setting", "aria-label": "Screen",
                button { class: "dark-switch",
                    onclick: move |_| SessionEvent::SwitchDark.applied(&session),
                    "Dark after sunset: {setting}"
                }
                p { class: "tagline", "Off by default. When on, the screen turns dark at sunset each day. The sunset time comes from the trip's dates and places. It needs no network." }
            }
        }
        .into_dyn_node()
    }
}
