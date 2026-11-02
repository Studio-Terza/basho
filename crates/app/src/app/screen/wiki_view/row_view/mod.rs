mod entry_where;
mod excerpt_view;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::place::NoteKind;
use guide::times::FactSource;
use wiki::EntryId;
use wiki::entry::{LoopRow, Row, StopRow};

use crate::app::screen::day_name::DayName;
use crate::app::screen::fact_view::FactView;
use crate::app::screen::linked_view::LinkedView;
use crate::app::screen::time_view::TimeView;
use crate::app::session::Session;
use entry_where::EntryWhere;
use excerpt_view::ExcerptView;

pub struct RowView<'view> {
    session: &'view Session,
    row: &'view Row,
}

impl<'view> RowView<'view> {
    pub fn new(session: &'view Session, row: &'view Row) -> Self {
        RowView { session, row }
    }

    fn entry(&self, id: EntryId) -> Element {
        let language = self.session.language();
        let itinerary = self.session.itinerary();
        rsx! {
            {self.session.wiki().entry(id).map(|entry| rsx! {
                p { class: "entry-row", lang: language.class(),
                    {LinkedView::entry(self.session, id, entry.name())}
                    {entry.home().map(|home| rsx! { span { class: "entry-where", " · {EntryWhere::new(home, itinerary)}" } })}
                }
            })}
        }
    }

    fn looped(&self, stop: &LoopRow) -> Element {
        let language = self.session.language();
        rsx! {
            div { class: "stop-row", lang: language.class(),
                p { class: "stop-name",
                    "{stop.number()} · "
                    {LinkedView::new(self.session, stop.name())}
                }
                p { class: "stop-when", {TimeView::new(stop.planned())} }
            }
        }
    }

    fn stop(&self, stop: &StopRow) -> Element {
        let language = self.session.language();
        let shown = stop
            .stop_entry()
            .filter(|entry| Some(*entry) != self.session.reader());
        rsx! {
            div { class: "stop-row", lang: language.class(),
                p { class: "stop-name",
                    {shown.map(|entry| LinkedView::entry(self.session, entry, stop.stop_name()))}
                    {stop.stop_entry().is_none().then(|| LinkedView::new(self.session, stop.stop_name()))}
                }
                p { class: "stop-when",
                    "{DayName::new(stop.visit_day())}"
                    {stop.expected().map(|time| rsx! { " · " {TimeView::new(time)} })}
                    {stop.closing().map(|cutoff| rsx! { " · " {TimeView::new(cutoff.listed_at())} })}
                    {stop.opening().map(|opening| rsx! { " · " strong { "{opening}" } })}
                }
                {stop.detail().filter(|fact| fact.kind() != NoteKind::Bookkeeping).map(|fact| rsx! {
                    p { class: "stop-detail", {FactView::new(fact, self.session).render_marked()} }
                })}
            }
        }
    }
}

impl IntoDynNode for RowView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let language = self.session.language();
        let row = self.row;
        rsx! {
            {row.text().map(|text| rsx! { p { lang: language.class(), {LinkedView::new(self.session, text)} } })}
            {row.note().map(|fact| rsx! {
                p { class: "wiki-note", lang: language.class(),
                    {(fact.kind() == NoteKind::Warning).then(|| rsx! { "⚠ " })}
                    {FactView::new(fact, self.session).render_marked()}
                }
            })}
            {row.sourced().map(|fact| FactView::new(fact.as_ref(), self.session).render_sourced())}
            {row.check_in().map(|time| rsx! {
                p { strong { {TimeView::worded(time, FactSource::CheckIn.word().lowered())} "." } }
            })}
            {row.entry().map(|id| self.entry(id))}
            {row.stop().map(|stop| self.stop(stop))}
            {row.looped().map(|stop| self.looped(stop))}
            {row.excerpt().map(|excerpt| rsx! { {ExcerptView::new(excerpt, self.session)} })}
            {row.disagreement().map(|disagreement| rsx! {
                div { class: "disagreement",
                    h3 { lang: language.class(), {LinkedView::new(self.session, AsRef::<guide::content::Content>::as_ref(disagreement))} }
                    p { lang: language.class(), {LinkedView::new(self.session, disagreement.detail())} }
                }
            })}
        }
        .into_dyn_node()
    }
}
