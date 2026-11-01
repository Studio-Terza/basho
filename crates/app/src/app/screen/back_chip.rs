use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use newtype::Newtype;

use crate::app::history::ScrollTop;
use crate::app::screen::origin_name::OriginName;
use crate::app::session::{Session, SessionEvent};
use crate::app::step::Toward;

#[derive(Newtype)]
pub struct BackChip<'view> {
    session: &'view Session,
}

impl IntoDynNode for BackChip<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let origin = self
            .session
            .history()
            .origin()
            .filter(|mark| mark.screen().wiki_entry().is_some());
        let language = self.session.language();
        origin
            .map(|mark| {
                let session = self.session.clone();
                rsx! {
                    button { class: "back-chip", lang: language.class(),
                        onclick: move |event| SessionEvent::TurnHistory { toward: Toward::Previous, top: ScrollTop::from(&event) }.applied(&session),
                        "‹ " {OriginName::new(self.session, mark.screen())}
                    }
                }
            })
            .into_dyn_node()
    }
}
