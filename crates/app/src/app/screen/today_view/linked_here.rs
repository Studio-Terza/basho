use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use wiki::EntryKey;
use wiki::entry::Heading;

use crate::app::screen::row_view::RowView;
use crate::app::session::Session;

pub struct LinkedHere<'view> {
    session: &'view Session,
    key: EntryKey,
}

impl<'view> LinkedHere<'view> {
    pub fn new(session: &'view Session, key: EntryKey) -> Self {
        LinkedHere { session, key }
    }
}

impl IntoDynNode for LinkedHere<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let wiki = self.session.wiki();
        let section = wiki
            .find(&self.key)
            .and_then(|id| wiki.entry(id))
            .and_then(|entry| {
                entry
                    .sections()
                    .iter()
                    .find(|section| section.heading() == Heading::LinkedHere)
            });
        section
            .map(|section| {
                rsx! {
                    div { class: "linked-here",
                        h3 { "{section.heading()}" }
                        {section.rows().iter().map(|row| rsx! { {RowView::new(self.session, row)} })}
                    }
                }
            })
            .into_dyn_node()
    }
}
