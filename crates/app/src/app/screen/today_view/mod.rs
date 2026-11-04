mod day_head;
mod day_nav;
mod linked_here;
mod step_view;
use crate::app::Position;
use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::content::Content;
use guide::end::End;
use guide::itinerary::{Caption, Day};
use newtype::Newtype;
use wiki::EntryKey;

use crate::app::screen::buffer_view::{BufferShape, BufferView};
use crate::app::screen::linked_view::LinkedView;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::scroll_anchor::ScrollAnchor;
use crate::app::screen::time_view::TimeView;
use crate::app::screen::unverified_mark::UnverifiedMark;
use crate::app::session::{Session, SessionEvent};
use crate::app::step::{DayPoint, GoalNumber, Step};
use crate::app::visit::{Focus, Landing};
use day_head::DayHead;
use day_nav::DayNav;
use linked_here::LinkedHere;
use step_view::StepView;

#[derive(Newtype)]
pub struct TodayView<'session> {
    session: &'session Session,
}

impl TodayView<'_> {
    fn day(&self, day: &Day) -> Element {
        let language = self.session.language();
        let guidance = self.session.guidance(day);
        let steps = Step::of_day(day);
        let today = self.session.day_at(self.session.now());
        let point = DayPoint::locate(&guidance, &steps).seen_from(day, today);
        let visit = self.session.visit();
        let home = self.session.clone();
        rsx! {
            div { class: "today",
                {(visit.landing() == Landing::Top).then(|| ScrollAnchor::new(visit))}
                {(visit.focus() != Focus::Now).then(|| rsx! {
                    button { class: "back-to-now", "aria-label": "Back to now",
                        onclick: move |_| SessionEvent::BackToNow.applied(&home),
                        "Back to now"
                    }
                })}
                {DayNav::new(self.session, day)}
                {DayHead::new(self.session, &guidance)}
                {LinkedHere::new(self.session, EntryKey::Day(day.number()))}
                {day.goals().iter().enumerate().map(|(position, goal)| {
                    let number = GoalNumber::from_position(Position::new(position));
                    let first = goal.leg(End::Start);
                    let last = goal.leg(End::Finish);
                    rsx! {
                        section { class: "goal", id: "{number.anchor()}",
                            {visit.landing().goal().filter(|focused| *focused == number).map(|_| ScrollAnchor::new(visit))}
                            h2 { class: "goal-head", lang: language.class(),
                                span { class: "goal-n", "GOAL {number}" }
                                " "
                                strong { {RubyView::new(&goal[Caption::Name])} {day.goal_evidence(goal).map(UnverifiedMark::new)} }
                                {day.leg(last).map(|leg| guidance.shifted(last, leg.listed(End::Finish))).map(|by| rsx! { span { class: "by", " by " {TimeView::new(by)} } })}
                            }
                            {guidance.buffer_through(first, last).map(|buffer| BufferView::new(&buffer, self.session.language()).render(BufferShape::Brief))}
                            p { class: "goal-why", lang: language.class(), {LinkedView::new(self.session, AsRef::<Content>::as_ref(goal))} }
                            {LinkedHere::new(self.session, EntryKey::Goal { day: day.number(), first })}
                            ol { class: "steps",
                                {steps.iter().filter(|step| goal.holds(step.index())).map(|step| rsx! {
                                    {StepView::new(self.session, &guidance, step, point.state_of(step.number()))}
                                })}
                            }
                        }
                    }
                })}
            }
        }
    }
}

impl IntoDynNode for TodayView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        self.session
            .day()
            .map_or_else(
                || rsx! { p { class: "error", "No days are loaded." } },
                |day| self.day(day),
            )
            .into_dyn_node()
    }
}
