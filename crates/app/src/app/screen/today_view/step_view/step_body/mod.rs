mod arrival_row;
mod ask_card;
mod briefing;
mod pin_button;
mod route_row;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::FactSource;
use guide::classed::Classed;
use guide::content::Content;
use guide::geo::{Distance, RouteLine};
use guide::guidance::Guidance;
use guide::leg::BranchLine;
use guide::leg::along::Along;
use guide::leg::conveyance::{Headsign, LineName, Platform, TrainKind};
use guide::leg_line::LegLine;
use guide::place::NoteKind;
use guide::words::Label;
use newtype::Counter;
use wiki::{EntryId, EntryKey, StopIndex};

use crate::app::history::ScrollTop;
use crate::app::screen::fact_view::FactView;
use crate::app::screen::linked_view::LinkedView;
use crate::app::screen::look::Look;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::time_view::TimeView;
use crate::app::screen::today_view::linked_here::LinkedHere;
use crate::app::screen::unverified_mark::UnverifiedMark;
use crate::app::screen::wording::{Rewording, Wording};
use crate::app::session::{Session, SessionEvent};
use crate::app::step::{Step, StepPart, StepState};
use arrival_row::ArrivalRow;
use ask_card::AskCard;
use briefing::Briefing;
use pin_button::PinButton;
use route_row::RouteRow;

pub struct StepBody<'view> {
    session: &'view Session,
    guidance: &'view Guidance<'view>,
    step: &'view Step<'view>,
    state: StepState,
}

impl<'view> StepBody<'view> {
    pub fn new(
        session: &'view Session,
        guidance: &'view Guidance<'view>,
        step: &'view Step<'view>,
        state: StepState,
    ) -> Self {
        StepBody {
            session,
            guidance,
            step,
            state,
        }
    }

    fn stop_name(&self, stop: &Along, index: StopIndex) -> Element {
        let key = EntryKey::Stop {
            day: self.guidance.day().number(),
            leg: self.step.index(),
            stop: index,
        };
        let entry = self.session.wiki().find(&key);
        entry.map_or_else(
            || rsx! { {LinkedView::new(self.session, AsRef::<Content>::as_ref(stop))} },
            |entry| LinkedView::entry(self.session, entry, stop.as_ref()),
        )
    }

    fn link_button(&self, entry: EntryId, label: Label) -> Element {
        let session = self.session.clone();
        rsx! {
            button { class: "place-link", onclick: move |event| SessionEvent::Follow { entry, top: ScrollTop::from(&event) }.applied(&session), "{label}" }
        }
    }

    fn briefing(&self, part: Briefing) -> Element {
        let language = self.session.language();
        let leg = self.step.leg();
        match part {
            Briefing::Train => rsx! {
                {leg.conveyance().train().map(|train| {
                    let headsign: &Headsign = train.as_ref();
                    let line: &LineName = train.as_ref();
                    let platform: &Platform = train.as_ref();
                    let kind: &TrainKind = train.as_ref();
                    rsx! {
                        dt { "TRAIN" }
                        dd { lang: language.class(),
                            p { strong { "{platform}" } }
                            p { strong { "「" {RubyView::new(headsign.as_ref())} "」" } }
                            p { {RubyView::new(line.as_ref())} " · {Label::from(kind)}" {UnverifiedMark::new(train.as_ref())} }
                        }
                    }
                })}
            },
            Briefing::Why => rsx! {
                dt { "WHY" }
                dd { lang: language.class(), {LinkedView::new(self.session, leg.line_text(LegLine::Why))} }
            },
            Briefing::Route => rsx! {
                {leg.route_reason().map(|reason| RouteRow::new(self.session, reason, language))}
            },
            Briefing::How => {
                let length = leg.length();
                let walked: &Content = leg
                    .road()
                    .map_or(leg.line_text(LegLine::Handrail), AsRef::as_ref);
                rsx! {
                    dt { "HOW" }
                    dd { lang: language.class(),
                        p { class: "way",
                            strong { {LinkedView::new(self.session, leg.line_text(LegLine::Way))} }
                            " · " {LinkedView::new(self.session, leg.line_text(LegLine::Heading))}
                            {(length > Distance::default()).then(|| rsx! { " · {length}" })}
                            " · {leg.minutes()}"
                        }
                        p { {LinkedView::reworded(self.session, walked, &Wording::new(walked).reworded(Rewording::ForWalker).ruby()).into_dyn_node()} }
                        {leg.notes().iter().filter(|fact| fact.kind() == NoteKind::Note).map(|fact| rsx! {
                            p { class: "note", {FactView::new(fact, self.session).render_marked()} }
                        })}
                    }
                }
            }
            Briefing::Ifs => {
                let index = self.step.index();
                let plans = self.guidance.plans(index);
                rsx! {
                    {plans.iter().filter(|plan| plan.leg() == index).map(|plan| {
                        let branch = plan.branch();
                        let pressing = Look::pressing(plan.pressing());
                        rsx! {
                            dt { class: pressing.class(), "IF" }
                            dd { class: pressing.class(), lang: language.class(),
                                {LinkedView::reworded(self.session, branch.line_text(BranchLine::Trigger), &Wording::new(branch.line_text(BranchLine::Trigger)).reworded(Rewording::ForWalker).reworded(Rewording::ConditionClause).ruby()).into_dyn_node()}
                                ": "
                                {LinkedView::reworded(self.session, branch.line_text(BranchLine::Backup), &Wording::new(branch.line_text(BranchLine::Backup)).reworded(Rewording::ForWalker).ruby()).into_dyn_node()}
                            }
                        }
                    })}
                }
            }
            Briefing::Asks => {
                let cards = leg.cards();
                let day = self.guidance.day();
                rsx! {
                    {(!cards.is_empty()).then(|| rsx! {
                        dt { "ASK" }
                        dd { class: "asks",
                            {cards.iter().map(|card| rsx! { {AskCard::new(card, day, self.session.language())} })}
                        }
                    })}
                }
            }
        }
    }

    fn arrival(&self, part: ArrivalRow) -> Element {
        let language = self.session.language();
        let leg = self.step.leg();
        match part {
            ArrivalRow::OnTheWay => {
                let along = leg.along();
                rsx! {
                    {(!along.is_empty()).then(|| rsx! {
                        dt { "ON THE WAY" }
                        dd {
                            ul { class: "along",
                                {along.iter().enumerate().map(|(position, stop)| rsx! {
                                    li { lang: language.class(),
                                        span { class: "at", {stop.expected().map(TimeView::new)} }
                                        span { class: "what",
                                            {self.stop_name(stop, StopIndex::new(position))}
                                            {stop.detail().filter(|fact| fact.kind() == NoteKind::Note).map(|fact| rsx! { span { class: "detail", " · " {FactView::new(fact, self.session).render_marked()} } })}
                                        }
                                    }
                                })}
                            }
                        }
                    })}
                }
            }
            ArrivalRow::There => {
                let day = self.guidance.day();
                let inn = day
                    .night()
                    .filter(|_| usize::from(self.step.index().incremented()) == day.legs().len());
                rsx! {
                    dt { "YOU ARE THERE" }
                    dd { lang: language.class(),
                        {Some(leg.see()).filter(|fact| fact.kind() == NoteKind::Note).map(|fact| rsx! {
                            p { "when you see: " {FactView::new(fact, self.session).render_marked()} }
                        })}
                        p { class: "then", {LinkedView::new(self.session, leg.line_text(LegLine::Then))} }
                        {inn.map(|night| rsx! {
                            p { strong { {TimeView::worded(night.listed_check_in(), FactSource::CheckIn.word().lowered())} "." } }
                            {night.notes().iter().filter(|fact| fact.kind() != NoteKind::Bookkeeping).map(|fact| rsx! {
                                p { {FactView::new(fact, self.session).render_marked()} }
                            })}
                        })}
                    }
                }
            }
            ArrivalRow::Warnings => rsx! {
                {leg.no_mapped_paths().map(|stretch| {
                    let line: &RouteLine = stretch.as_ref();
                    rsx! {
                        dt { class: "warn", "⚠" }
                        dd { lang: language.class(), strong { "No mapped path" } " for {line.length()}. The line here is on no known road." }
                    }
                })}
                {std::iter::once(leg.see())
                    .chain(leg.along().iter().filter_map(|stop| stop.detail()))
                    .chain(
                        self.step
                            .index()
                            .previous()
                            .into_iter()
                            .flat_map(|_| leg.notes()),
                    )
                    .filter(|fact| fact.kind() == NoteKind::Warning)
                    .map(|fact| rsx! {
                    dt { class: "warn", "⚠" }
                    dd { lang: language.class(), {FactView::new(fact, self.session).render_marked()} }
                })}
            },
            ArrivalRow::PlaceLink => {
                let wiki = self.session.wiki();
                let day = self.guidance.day();
                let place = leg
                    .place()
                    .and_then(|slug| wiki.find(&EntryKey::Place(slug.clone())));
                let evening = day
                    .night()
                    .filter(|_| usize::from(self.step.index().incremented()) == day.legs().len())
                    .and_then(|_| wiki.find(&EntryKey::Evening(day.number())));
                rsx! {
                    {place.map(|entry| self.link_button(entry, Label::new("Read about this place ›")))}
                    {evening.map(|entry| self.link_button(entry, Label::new("This evening ›")))}
                    {LinkedHere::new(self.session, EntryKey::Step { day: day.number(), leg: self.step.index() })}
                }
            }
        }
    }
}

impl IntoDynNode for StepBody<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        match self.step.part() {
            StepPart::Walk => rsx! {
                dl { class: "rows",
                    {self.briefing(Briefing::Why)}
                    {self.briefing(Briefing::Route)}
                    {self.briefing(Briefing::How)}
                    {self.arrival(ArrivalRow::OnTheWay)}
                    {self.arrival(ArrivalRow::There)}
                    {self.arrival(ArrivalRow::Warnings)}
                    {self.briefing(Briefing::Ifs)}
                    {self.briefing(Briefing::Asks)}
                }
                {self.arrival(ArrivalRow::PlaceLink)}
                {PinButton::new(self.session, self.guidance.day(), self.step, self.state)}
            },
            StepPart::Board => rsx! {
                dl { class: "rows",
                    {self.briefing(Briefing::Train)}
                    {self.briefing(Briefing::Why)}
                    {self.briefing(Briefing::How)}
                    {self.briefing(Briefing::Ifs)}
                    {self.briefing(Briefing::Asks)}
                }
                {self.arrival(ArrivalRow::PlaceLink)}
                {PinButton::new(self.session, self.guidance.day(), self.step, self.state)}
            },
            StepPart::Alight => rsx! {
                dl { class: "rows",
                    {self.arrival(ArrivalRow::OnTheWay)}
                    {self.arrival(ArrivalRow::There)}
                    {self.arrival(ArrivalRow::Warnings)}
                }
                {PinButton::new(self.session, self.guidance.day(), self.step, self.state)}
            },
        }
        .into_dyn_node()
    }
}
