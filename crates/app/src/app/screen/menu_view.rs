use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::end::End;
use guide::english_name::EnglishName;
use guide::itinerary::{Caption, Day, DayNumber};
use menu::PoseStyle;
use newtype::Newtype;
use wiki::EntryKey;

use crate::app::history::ScrollTop;
use crate::app::screen::Screen;
use crate::app::screen::day_name::DayName;
use crate::app::screen::look::Look;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::scroll_anchor::ScrollAnchor;
use crate::app::screen::time_view::TimeView;
use crate::app::screen::touch_surface::TouchSurface;
use crate::app::session::{Landing, Session, SessionEvent};
use crate::app::step::GoalNumber;

#[derive(Newtype)]
pub struct MenuView<'session> {
    session: &'session Session,
}

impl MenuView<'_> {
    fn day(&self, day: &Day, today: Option<DayNumber>) -> Element {
        let language = self.session.language();
        let session = self.session.clone();
        let number = day.number();
        let name = DayName::new(number);
        let current = today.filter(|today| *today == number);
        let guidance = self.session.guidance(day);
        rsx! {
            li { class: Look::menu_day(current).class(), id: "menu-day-{number}",
                {current.map(|_| ScrollAnchor::new(self.session.visit()))}
                button { class: "day-row", "aria-label": "{name}",
                    onclick: move |event| {
                        SessionEvent::Remember(ScrollTop::from(&event)).applied(&session);
                        session.jump(number, Landing::Top);
                    },
                    span { class: "day-when",
                        {current.map(|_| rsx! { span { class: "mark", "aria-hidden": "true", "▶" } })}
                        "{name} · {day.date().short()}"
                    }
                    span { class: "day-what", lang: language.class(), {RubyView::new(&day[Caption::Name])} }
                }
                ol { class: "menu-goals",
                    {day.goals().iter().enumerate().map(|(position, goal)| {
                        let session = self.session.clone();
                        let goal_number = GoalNumber::from_position(position);
                        let last = goal.leg(End::Finish);
                        rsx! {
                            li {
                                button { class: "goal-row", lang: language.class(), "aria-label": "{name} goal {goal_number}",
                                    onclick: move |event| {
                                        SessionEvent::Remember(ScrollTop::from(&event)).applied(&session);
                                        session.jump(number, Landing::Goal(goal_number));
                                    },
                                    span { class: "goal-n", "{goal_number}" }
                                    strong { {RubyView::new(&goal[Caption::Name])} }
                                    {day.leg(last).map(|leg| guidance.shifted(last, leg.listed(End::Finish))).map(|by| rsx! { span { class: "by", "by " {TimeView::new(by)} } })}
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}

impl IntoDynNode for MenuView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let pose: PoseStyle = self.session.menu_style();
        let closer = self.session.clone();
        let reporter = self.session.clone();
        let lister = self.session.clone();
        let held = self.session.report_book().reports().len();
        let today = self.session.day_at(self.session.now()).map(Day::number);
        let picker = self.session.clone();
        let trip = AsRef::<EnglishName>::as_ref(self.session.itinerary().head());
        let scrim = rsx! {
            div { class: "menu-scrim", style: "{pose}", onclick: move |_| SessionEvent::CloseMenu.applied(&closer) }
        };
        let index = self.session.wiki().find(&EntryKey::Index);
        let opener = self.session.clone();
        let panel = rsx! {
            nav { class: "menu", "data-overlay": "menu", "aria-label": "Day menu", style: "{pose}",
                div { class: "menu-head",
                    h2 { class: "menu-title", "Days" }
                    button { class: "menu-trip",
                        onclick: move |event| {
                            SessionEvent::CloseMenu.applied(&picker);
                            SessionEvent::OpenTab { screen: Screen::Trip, top: ScrollTop::from(&event) }.applied(&picker);
                        },
                        "Trip: {trip}"
                    }
                    div { class: "menu-tools",
                        {index.map(|index| rsx! {
                            button { class: "report-tool menu-index",
                                onclick: move |event| {
                                    SessionEvent::CloseMenu.applied(&opener);
                                    SessionEvent::Follow { entry: index, top: ScrollTop::from(&event) }.applied(&opener);
                                },
                                "Index"
                            }
                        })}
                        button { class: "report-tool report-bug",
                            onclick: move |_| SessionEvent::CaptureReport.applied(&reporter),
                            "Report a bug"
                        }
                        button { class: "report-tool reports-open",
                            onclick: move |_| SessionEvent::ShowReports.applied(&lister),
                            "Reports ({held})"
                        }
                    }
                }
                ol { class: "menu-days",
                    {self.session.itinerary().days().iter().map(|day| self.day(day, today))}
                }
            }
        };
        rsx! {
            {TouchSurface::Menu.wrap(self.session, scrim)}
            {TouchSurface::Menu.wrap(self.session, panel)}
        }
        .into_dyn_node()
    }
}
