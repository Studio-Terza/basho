use dioxus::prelude::*;
use guide::FactSource;
use guide::classed::Classed;
use guide::content::Content;
use guide::geo::RouteLine;
use guide::itinerary::day::Night;
use guide::leg::along::Along;
use guide::leg_line::LegLine;
use guide::place::NoteKind;
use guide::words::Label;
use newtype::Counter;
use wiki::{EntryId, EntryKey, StopIndex};

use crate::app::history::ScrollTop;
use crate::app::screen::fact_view::FactView;
use crate::app::screen::linked_view::LinkedView;
use crate::app::screen::time_view::TimeView;
use crate::app::screen::today_view::linked_here::LinkedHere;
use crate::app::session::SessionEvent;

use super::StepBody;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrivalRow {
    OnTheWay,
    There,
    Warnings,
    PlaceLink,
}

impl ArrivalRow {
    pub fn shown(self, body: &StepBody<'_>) -> Element {
        match self {
            ArrivalRow::OnTheWay => rsx! {
                {body.leg.along().first().map(|_| rsx! {
                    dt { "ON THE WAY" }
                    dd {
                        ul { class: "along",
                            {body.leg.along().iter().enumerate().map(|(position, stop)| rsx! {
                                li { lang: body.language.class(),
                                    span { class: "at", {stop.expected().map(TimeView::new)} }
                                    span { class: "what",
                                        {Self::stop_name(body, stop, StopIndex::new(position))}
                                        {stop.detail().filter(|fact| fact.kind() == NoteKind::Note).map(|fact| rsx! { span { class: "detail", " · " {FactView::new(fact, body.session).render_marked()} } })}
                                    }
                                }
                            })}
                        }
                    }
                })}
            },
            ArrivalRow::There => rsx! {
                dt { "YOU ARE THERE" }
                dd { lang: body.language.class(),
                    {Some(body.leg.see()).filter(|fact| fact.kind() == NoteKind::Note).map(|fact| rsx! {
                        p { "when you see: " {FactView::new(fact, body.session).render_marked()} }
                    })}
                    p { class: "then", {LinkedView::new(body.session, body.leg.line_text(LegLine::Then))} }
                    {Self::closing_night(body).map(|night| rsx! {
                        p { strong { {TimeView::worded(night.listed_check_in(), FactSource::CheckIn.word().lowered())} "." } }
                        {night.notes().iter().filter(|fact| fact.kind() != NoteKind::Bookkeeping).map(|fact| rsx! {
                            p { {FactView::new(fact, body.session).render_marked()} }
                        })}
                    })}
                }
            },
            ArrivalRow::Warnings => rsx! {
                {body.leg.no_mapped_paths().map(|stretch| {
                    let line: &RouteLine = stretch.as_ref();
                    rsx! {
                        dt { class: "warn", "⚠" }
                        dd { lang: body.language.class(), strong { "No mapped path" } " for {line.length()}. The line here is on no known road." }
                    }
                })}
                {std::iter::once(body.leg.see())
                    .chain(body.leg.along().iter().filter_map(|stop| stop.detail()))
                    .chain(
                        body.step
                            .index()
                            .previous()
                            .into_iter()
                            .flat_map(|_| body.leg.notes()),
                    )
                    .filter(|fact| fact.kind() == NoteKind::Warning)
                    .map(|fact| rsx! {
                    dt { class: "warn", "⚠" }
                    dd { lang: body.language.class(), {FactView::new(fact, body.session).render_marked()} }
                })}
            },
            ArrivalRow::PlaceLink => rsx! {
                {body.leg.place().and_then(|slug| body.session.wiki().find(&EntryKey::Place(slug.clone()))).map(|entry| Self::link_button(body, entry, Label::new("Read about this place ›")))}
                {Self::closing_night(body).and_then(|_| body.session.wiki().find(&EntryKey::Evening(body.guidance.day().number()))).map(|entry| Self::link_button(body, entry, Label::new("This evening ›")))}
                {LinkedHere::new(body.session, EntryKey::Step { day: body.guidance.day().number(), leg: body.step.index() })}
            },
        }
    }

    fn stop_name(body: &StepBody<'_>, stop: &Along, index: StopIndex) -> Element {
        let key = EntryKey::Stop {
            day: body.guidance.day().number(),
            leg: body.step.index(),
            stop: index,
        };
        let entry = body.session.wiki().find(&key);
        entry.map_or_else(
            || rsx! { {LinkedView::new(body.session, AsRef::<Content>::as_ref(stop))} },
            |entry| LinkedView::entry(body.session, entry, stop.as_ref()),
        )
    }

    fn link_button(body: &StepBody<'_>, entry: EntryId, label: Label) -> Element {
        let session = body.session.clone();
        rsx! {
            button { class: "place-link", onclick: move |event| SessionEvent::Follow { entry, top: ScrollTop::from(&event) }.applied(&session), "{label}" }
        }
    }

    fn closing_night<'view>(body: &StepBody<'view>) -> Option<&'view Night> {
        body.guidance.day().night().filter(|_| {
            usize::from(body.step.index().incremented()) == body.guidance.day().legs().len()
        })
    }
}
