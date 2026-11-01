use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::content::Content;
use guide::place::RubyText;
use wiki::EntryId;

use crate::app::history::ScrollTop;
use crate::app::screen::ruby_view::RubyView;
use crate::app::session::{Session, SessionEvent};

pub struct LinkedView<'text> {
    session: &'text Session,
    content: &'text Content,
    shown: &'text RubyText,
}

impl<'text> LinkedView<'text> {
    pub fn new(session: &'text Session, content: &'text Content) -> Self {
        Self::reworded(session, content, content.shown())
    }

    pub fn reworded(
        session: &'text Session,
        content: &'text Content,
        shown: &'text RubyText,
    ) -> Self {
        LinkedView {
            session,
            content,
            shown,
        }
    }

    pub fn entry(session: &Session, entry: EntryId, text: &RubyText) -> Element {
        let follower = session.clone();
        rsx! {
            a { class: "link", role: "link",
                onclick: move |event| SessionEvent::Follow { entry, top: ScrollTop::from(&event) }.applied(&follower),
                {RubyView::new(text)}
            }
        }
    }
}

impl IntoDynNode for LinkedView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let runs =
            self.session
                .wiki()
                .names()
                .runs(self.content, self.shown, self.session.reader());
        rsx! {
            {runs.iter().map(|run| run.entry().map_or_else(
                || rsx! { {RubyView::new(run.text())} },
                |entry| Self::entry(self.session, entry, run.text()),
            ))}
        }
        .into_dyn_node()
    }
}
