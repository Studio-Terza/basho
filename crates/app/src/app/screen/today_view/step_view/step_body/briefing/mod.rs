mod ask_card;
mod route_row;

use dioxus::prelude::*;
use guide::classed::Classed;
use guide::content::Content;
use guide::geo::Distance;
use guide::leg::BranchLine;
use guide::leg::conveyance::{Headsign, LineName, Platform, TrainKind};
use guide::leg_line::LegLine;
use guide::place::NoteKind;
use guide::words::Label;

use crate::app::screen::fact_view::FactView;
use crate::app::screen::linked_view::LinkedView;
use crate::app::screen::look::Look;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::unverified_mark::UnverifiedMark;
use crate::app::screen::wording::{Rewording, Wording};

use super::StepBody;
use ask_card::AskCard;
use route_row::RouteRow;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Briefing {
    Train,
    Why,
    Route,
    How,
    Ifs,
    Asks,
}

impl Briefing {
    pub fn shown(self, body: &StepBody<'_>) -> Element {
        match self {
            Briefing::Train => rsx! {
                {body.leg.conveyance().train().map(|train| {
                    let headsign: &Headsign = train.as_ref();
                    let line: &LineName = train.as_ref();
                    let platform: &Platform = train.as_ref();
                    let kind: &TrainKind = train.as_ref();
                    rsx! {
                        dt { "TRAIN" }
                        dd { lang: body.language.class(),
                            p { strong { "{platform}" } }
                            p { strong { "「" {RubyView::new(headsign.as_ref())} "」" } }
                            p { {RubyView::new(line.as_ref())} " · {Label::from(kind)}" {UnverifiedMark::new(train.as_ref())} }
                        }
                    }
                })}
            },
            Briefing::Why => rsx! {
                dt { "WHY" }
                dd { lang: body.language.class(), {LinkedView::new(body.session, body.leg.line_text(LegLine::Why))} }
            },
            Briefing::Route => rsx! {
                {body.leg.route_reason().map(|reason| RouteRow::new(body.session, reason, body.language))}
            },
            Briefing::How => rsx! {
                dt { "HOW" }
                dd { lang: body.language.class(),
                    p { class: "way",
                        strong { {LinkedView::new(body.session, body.leg.line_text(LegLine::Way))} }
                        " · " {LinkedView::new(body.session, body.leg.line_text(LegLine::Heading))}
                        {Some(body.leg.length()).filter(|length| *length > Distance::default()).map(|length| rsx! { " · {length}" })}
                        " · {body.leg.minutes()}"
                    }
                    p { {LinkedView::reworded(body.session, Self::walked(body), &Wording::new(Self::walked(body)).reworded(Rewording::ForWalker).ruby()).into_dyn_node()} }
                    {body.leg.notes().iter().filter(|fact| fact.kind() == NoteKind::Note).map(|fact| rsx! {
                        p { class: "note", {FactView::new(fact, body.session).render_marked()} }
                    })}
                }
            },
            Briefing::Ifs => rsx! {
                {body.guidance.plans(body.step.index()).into_iter().filter(|plan| plan.leg() == body.step.index()).map(|plan| {
                    let branch = plan.branch();
                    let pressing = Look::pressing(plan.pressing());
                    rsx! {
                        dt { class: pressing.class(), "IF" }
                        dd { class: pressing.class(), lang: body.language.class(),
                            {LinkedView::reworded(body.session, branch.line_text(BranchLine::Trigger), &Wording::new(branch.line_text(BranchLine::Trigger)).reworded(Rewording::ForWalker).reworded(Rewording::ConditionClause).ruby()).into_dyn_node()}
                            ": "
                            {LinkedView::reworded(body.session, branch.line_text(BranchLine::Backup), &Wording::new(branch.line_text(BranchLine::Backup)).reworded(Rewording::ForWalker).ruby()).into_dyn_node()}
                        }
                    }
                })}
            },
            Briefing::Asks => rsx! {
                {body.leg.cards().first().map(|_| rsx! {
                    dt { "ASK" }
                    dd { class: "asks",
                        {body.leg.cards().iter().map(|card| rsx! { {AskCard::new(card, body.guidance.day(), body.language)} })}
                    }
                })}
            },
        }
    }

    fn walked<'view>(body: &StepBody<'view>) -> &'view Content {
        body.leg
            .road()
            .map_or(body.leg.line_text(LegLine::Handrail), AsRef::as_ref)
    }
}
