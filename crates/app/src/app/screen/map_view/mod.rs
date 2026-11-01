mod arrow_drawing;
mod browse_map;
mod day_map;
mod day_notice;
mod narration;
mod picked_line;
mod status_text;
mod stepper;
use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::geo::Distance;
use guide::guidance::{Guidance, Phase, PositionSource};
use guide::itinerary::{Caption, Day};
use guide::leg::Mode;
use guide::leg_line::LegLine;
use guide::readout::Emphasis;
use layout::measure::Ground;
use map_plan::{MapPlan, MapSent, MapSight, MapUpdate};
use newtype::Newtype;

use crate::app::screen::buffer_view::{BufferShape, BufferView};
use crate::app::screen::day_name::DayName;
use crate::app::screen::fact_view::FactView;
use crate::app::screen::look::Look;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::step_title::StepTitle;
use crate::app::screen::time_view::TimeView;
use crate::app::screen::unverified_mark::UnverifiedMark;
use crate::app::screen::wording::{Rewording, Wording};
use crate::app::session::{Session, SessionEvent};
use crate::app::step::{DayPoint, Step, Toward};
use arrow_drawing::ArrowDrawing;
use browse_map::BrowseMap;
use day_map::DayMapView;
use day_notice::DayNotice;
use narration::Narration;
use picked_line::PickedLine;
use status_text::StatusText;
use stepper::Stepper;

#[derive(Newtype)]
pub struct MapView<'session> {
    session: &'session Session,
}

impl MapView<'_> {
    fn today(&self, day: &Day) -> Element {
        let guidance = self.session.guidance(day);
        let steps = Step::of_day(day);
        let step = DayPoint::locate(&guidance, &steps)
            .current()
            .and_then(|number| steps.iter().find(|step| step.number() == number));
        rsx! {
            div { class: "map-screen",
                {guidance.buffer().map(|buffer| BufferView::new(&buffer, self.session.language()).render(BufferShape::Full))}
                {DayNotice::new(Narration::new(&guidance, self.session.language()))}
                {self.top_line(&guidance, step)}
                {step.map(|step| self.recovery(&guidance, step))}
                {PickedLine::new(self.session)}
                div { class: "map-wrap", {DayMapView::new(self.session)} }
                {self.status_bar(&guidance)}
            }
        }
    }

    fn top_line(&self, guidance: &Guidance<'_>, step: Option<&Step<'_>>) -> Element {
        let language = self.session.language();
        let opener = self.session.clone();
        let day = guidance.day();
        rsx! {
            div { class: "top-row",
                {Stepper::new(self.session).render(Toward::Previous)}
                button { class: "top-line", onclick: move |_| SessionEvent::ShowCurrentStep.applied(&opener),
                    {step.map_or_else(
                        || rsx! {
                            p { class: "step-title", lang: language.class(), "✓ Day done. "
                                {day.night().map(|night| rsx! { "Sleep at " {RubyView::new(&night[Caption::Name])} {UnverifiedMark::new(night.as_ref())} "." })}
                            }
                            p { class: "time-check", "{DayName::new(day.number())} · {day.date().short()}" }
                        },
                        |step| rsx! {
                            p { class: "step-title", lang: language.class(), "▶ {step.number()} " {StepTitle::new(step)} }
                            p { class: "time-check", lang: language.class(),
                                {guidance.time_check().map_or_else(
                                    || rsx! { "{DayName::new(day.number())} · {day.date().short()} · by " {TimeView::new(guidance.shifted(step.index(), step.due()))} },
                                    |check| rsx! {
                                        span { class: Look::time_check(check.emphasis()).class(), {check.late_by().map(|over| rsx! { "LATE {over} · " })} "by " {TimeView::new(check.expected_by())} "{check.rest()}" }
                                        {check.late_by().map_or_else(
                                            || rsx! {
                                                {guidance.leg().filter(|leg| leg.mode() == Mode::Walk && guidance.phase() == Phase::Moving && guidance.source() == PositionSource::Gps).map(|_| rsx! { " · {guidance.left()} to go" })}
                                            },
                                            |_| rsx! { " · You should see " {RubyView::new(StepTitle::new(step).place())} "." },
                                        )}
                                    },
                                )}
                            }
                        },
                    )}
                }
                {Stepper::new(self.session).render(Toward::Next)}
            }
        }
    }

    fn recovery(&self, guidance: &Guidance<'_>, step: &Step<'_>) -> Element {
        let language = self.session.language();
        let leg = step.leg();
        let late = guidance
            .time_check()
            .filter(|check| check.late_by().is_some());
        let off = guidance
            .status()
            .filter(|status| status.emphasis() == Emphasis::Inverted);
        let lost = guidance.source().lost_for(self.session.now());
        let adrift = off.map(drop).or(lost.map(drop));
        let since = guidance.recovery().and_then(|recovery| recovery.fix_time());
        let beside = Distance::from(Ground::Beside);
        let ahead = guidance
            .arrow()
            .filter(|_| adrift.is_none())
            .map(|arrow| arrow.pointer().bearing().compass_point().words());
        rsx! {
            {guidance.recovery().map(|recovery| rsx! {
                section { class: "recovery", lang: language.class(),
                    {recovery.route().filter(|route| off.is_some() && route.distance() >= beside).map(|route| rsx! {
                        p { strong { "The route is {route.distance()} {route.bearing().compass_point().words()} of you." }
                            {since.map(|time| rsx! { " (from " {TimeView::new(time)} ")" })}
                        }
                    })}
                    {ahead.map(|words| rsx! {
                        p { strong { "You are on the route. Walk {guidance.left()} {words} to " {RubyView::new(StepTitle::new(step).place())} "." } }
                    })}
                    {adrift.map(|()| {
                        let sights: Vec<_> = recovery.toward().iter().filter(|sight| sight.pointer().distance() >= beside).collect();
                        rsx! {
                            {(!sights.is_empty()).then(|| rsx! {
                                p { class: "row-label", "WALK TOWARD" }
                                ul { class: "toward",
                                    {sights.iter().map(|sight| rsx! {
                                        li {
                                            "◆ " strong { {RubyView::new(sight.name())} }
                                            " · {sight.pointer().distance()} {sight.pointer().bearing().compass_point()}"
                                            {sight.note().map(|note| rsx! { " · " {RubyView::new(note)} })}
                                        }
                                    })}
                                }
                            })}
                        }
                    })}
                    {late.map(|_| rsx! { p { "Look for: " {FactView::new(leg.see(), self.session).render_marked()} } })}
                    p { span { class: "row-label", "THEN " } "Walk on " {RubyView::new(&Wording::new(leg.line_text(LegLine::Way)).reworded(Rewording::ObjectPhrase).ruby()).into_dyn_node()} ", " {RubyView::new(leg.line_text(LegLine::Heading))} ", to " {RubyView::new(StepTitle::new(step).place())} "." }
                }
            })}
        }
    }

    fn status_bar(&self, guidance: &Guidance<'_>) -> Element {
        let facing = self.session.heading();
        rsx! {
            div { class: "status-bar",
                {self.session.location_access().problem().map_or_else(
                    || rsx! { {StatusText::new(Narration::new(guidance, self.session.language()))} },
                    |problem| rsx! { p { class: "status alarm location-off", role: "alert", "{problem}" } },
                )}
                {guidance.arrow().map_or_else(
                    || rsx! { div { class: "arrow none", "aria-label": "No arrow without GPS", "–" } },
                    |arrow| {
                        rsx! {
                            div { class: "arrow", role: "img", "aria-label": "Walk this way",
                                {ArrowDrawing::new(arrow.angle(facing).turn())}
                                span { class: "arrow-words",
                                    "{arrow.pointer().bearing().compass_point().words()}"
                                    {facing.is_none().then(|| rsx! { br {} "north is up" })}
                                    {arrow.fix_time().map(|time| rsx! { br {} "(from " {TimeView::new(time)} ")" })}
                                }
                            }
                        }
                    },
                )}
            }
        }
    }
}

impl MapView<'_> {
    pub fn update_of(self, sent: &mut MapSent) -> Option<MapUpdate> {
        let session = self.session;
        let spot = session.visit().focus().spot();
        let today = session.day_at(session.now());
        let day = spot
            .and_then(|spot| session.itinerary().day_number(spot.day()))
            .or(today)?;
        let shown = session.guidance(day);
        let walker = today.map_or_else(|| session.guidance(day), |today| session.guidance(today));
        let steps = Step::of_day(day);
        let step = spot.map_or_else(
            || DayPoint::locate(&shown, &steps).current(),
            |spot| Some(spot.step()),
        );
        let step = step
            .and_then(|number| steps.iter().find(|step| step.number() == number))
            .or(steps.last())?;
        let sight = spot.map_or_else(
            || MapSight::live(&shown),
            |_| MapSight::browsing(&walker, step.index()),
        );
        let plan = MapPlan::new(session.overview(), &shown, sight, session.map_level())?;
        sent.change_for(
            &plan,
            &StepTitle::new(step).place_words(),
            &session.trail(),
            session.heading(),
        )
    }
}

impl IntoDynNode for MapView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        self.session
            .visit()
            .focus()
            .spot()
            .and_then(|spot| BrowseMap::new(self.session, spot).render_found())
            .unwrap_or_else(|| {
                self.session.day_at(self.session.now()).map_or_else(
                    || rsx! { p { class: "error", "No days are loaded." } },
                    |day| self.today(day),
                )
            })
            .into_dyn_node()
    }
}
