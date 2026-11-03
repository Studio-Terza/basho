use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;

use crate::app::screen::ruby_view::RubyView;
use crate::app::session::Session;
use crate::app::visit::Screen;

pub struct OriginName<'view> {
    session: &'view Session,
    screen: Screen,
}

impl<'view> OriginName<'view> {
    pub fn new(session: &'view Session, screen: Screen) -> Self {
        OriginName { session, screen }
    }
}

impl IntoDynNode for OriginName<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        self.screen
            .wiki_entry()
            .and_then(|id| self.session.wiki().entry(id))
            .map_or_else(
                || rsx! { "{self.screen.tab()}" },
                |entry| rsx! { {RubyView::new(entry.name())} },
            )
            .into_dyn_node()
    }
}
