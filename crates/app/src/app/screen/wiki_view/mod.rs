mod section_anchor;
mod section_index;
mod wiki_bar;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use wiki::EntryId;
use wiki::entry::Entry;

use crate::app::screen::linked_view::LinkedView;
use crate::app::screen::row_view::RowView;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::unverified_mark::UnverifiedMark;
use crate::app::session::Session;
use section_anchor::SectionAnchor;
use section_index::SectionIndex;
use wiki_bar::WikiBar;

pub struct WikiView<'view> {
    session: &'view Session,
    id: EntryId,
    entry: &'view Entry,
}

impl<'view> WikiView<'view> {
    pub fn new(session: &'view Session, id: EntryId, entry: &'view Entry) -> Self {
        WikiView { session, id, entry }
    }
}

impl IntoDynNode for WikiView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let language = self.session.language();
        let entry = self.entry;
        rsx! {
            article { class: "wiki", key: "{usize::from(self.id)}",
                {WikiBar::new(self.session, entry)}
                header {
                    p { class: "wiki-kind", "{entry.kind()}" }
                    h1 { lang: language.class(), {RubyView::new(entry.name())} {entry.evidence().map(UnverifiedMark::new)} }
                    {entry.english().map(|english| rsx! { p { class: "english", "{english}" } })}
                    {entry.tagline().map(|tagline| rsx! { p { class: "tagline", {LinkedView::new(self.session, tagline)} } })}
                }
                {entry.sections().iter().enumerate().map(|(position, section)| rsx! {
                    section { id: "{SectionAnchor::new(SectionIndex::new(position))}",
                        h2 { "{section.heading()}" }
                        {section.rows().iter().map(|row| rsx! { {RowView::new(self.session, row)} })}
                    }
                })}
            }
        }
        .into_dyn_node()
    }
}
