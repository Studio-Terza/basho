use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::Language;
use guide::classed::Classed;
use guide::leg::conveyance::RouteReason;
use guide::words::Label;

use crate::app::screen::linked_view::LinkedView;
use crate::app::screen::unverified_mark::UnverifiedMark;
use crate::app::session::Session;

pub struct RouteRow<'row> {
    session: &'row Session,
    reason: &'row RouteReason,
    language: Language,
}

impl<'row> RouteRow<'row> {
    pub fn new(session: &'row Session, reason: &'row RouteReason, language: Language) -> Self {
        RouteRow {
            session,
            reason,
            language,
        }
    }

    fn recorded(&self, label: Label) -> Element {
        rsx! {
            strong { "{label}" }
            {self.reason.referent().map(|referent| rsx! { " · " {LinkedView::new(self.session, referent.as_ref())} })}
            {self.reason.evidence().map(UnverifiedMark::new)}
        }
    }
}

impl IntoDynNode for RouteRow<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let shown = self.reason.kind().map_or_else(
            || rsx! { strong { "No reason recorded" } },
            |kind| self.recorded(Label::from(&kind)),
        );
        rsx! {
            dt { "WHY THIS WAY" }
            dd { lang: self.language.class(), {shown} }
        }
        .into_dyn_node()
    }
}
