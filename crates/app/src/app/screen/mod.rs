mod appearance;
mod back_chip;
mod buffer_view;
mod dark_switch;
mod day_name;
mod fact_view;
mod linked_view;
mod look;
mod map_view;
mod menu_view;
mod origin_name;
mod reports_view;
mod ruby_view;
mod scroll_anchor;
mod step_title;
mod time_view;
mod today_view;
mod touch_surface;
mod trip_view;
mod turning;
mod unverified_mark;
mod wiki_view;
pub mod wording;

use std::fmt;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::english_name::EnglishName;
use guide::map_frame::Pixels;
use layout::RootStyle;
use layout::scale::Viewport;
use map_plan::{MapSent, MapUpdate};
use wiki::EntryId;

use crate::app::history::ScrollTop;
use crate::app::session::{PageShift, Session, SessionEvent, Visit};
pub use appearance::Appearance;
use back_chip::BackChip;
use dark_switch::DarkSwitch;
use look::Look;
use map_view::MapView;
use menu::Menu;
use menu_view::MenuView;
use reports_view::ReportsView;
use today_view::TodayView;
use touch_surface::TouchSurface;
use trip_view::TripView;
pub use turning::Turning;
use wiki_view::WikiView;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Screen {
    #[default]
    Map,
    Today,
    Wiki(EntryId),
    Reports,
    Trip,
}

impl Screen {
    pub const STYLE: Asset = asset!("/assets/main.css");
    const TABS: [Screen; 2] = [Screen::Map, Screen::Today];

    #[must_use]
    pub fn wiki_entry(&self) -> Option<EntryId> {
        match self {
            Screen::Wiki(id) => Some(*id),
            Screen::Map | Screen::Today | Screen::Reports | Screen::Trip => None,
        }
    }

    #[must_use]
    pub fn turning(&self) -> Turning {
        match self {
            Screen::Map => Turning::Steps,
            Screen::Today => Turning::Days,
            Screen::Wiki(_) => Turning::History,
            Screen::Reports | Screen::Trip => Turning::Still,
        }
    }

    fn tab_label(&self) -> &'static str {
        match self {
            Screen::Map => "Map",
            Screen::Today | Screen::Wiki(_) | Screen::Reports | Screen::Trip => "Today",
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Screen::Map => formatter.write_str("map"),
            Screen::Today => formatter.write_str("today"),
            Screen::Wiki(id) => write!(formatter, "wiki page {}", usize::from(*id)),
            Screen::Reports => formatter.write_str("bug reports"),
            Screen::Trip => formatter.write_str("trip"),
        }
    }
}

pub struct ScreenFrame<'view> {
    screen: Screen,
    session: &'view Session,
}

impl<'view> ScreenFrame<'view> {
    const JUMP_TOP: &str = "window.scrollTo(0, await dioxus.recv())";

    #[must_use]
    pub fn new(screen: Screen, session: &'view Session) -> Self {
        ScreenFrame { screen, session }
    }

    pub fn map_update(session: &Session, sent: &mut MapSent) -> Option<MapUpdate> {
        MapView::new(session).update_of(sent)
    }

    fn body(&self) -> Element {
        let session = self.session;
        let entry = self.screen.wiki_entry().and_then(|id| {
            session
                .wiki()
                .entry(id)
                .map(|entry| WikiView::new(session, id, entry))
        });
        entry.map_or_else(
            || match self.screen {
                Screen::Map => rsx! { {MapView::new(session)} },
                Screen::Today | Screen::Wiki(_) => rsx! { {TodayView::new(session)} },
                Screen::Trip => Self::trip_page(session),
                Screen::Reports => rsx! { {ReportsView::new(session)} },
            },
            |view| rsx! { {view} },
        )
    }

    fn trip_page(session: &Session) -> Element {
        let back = session.clone();
        let name = AsRef::<EnglishName>::as_ref(session.itinerary().head());
        rsx! { {TripView::new(name, move |event| back.leave(ScrollTop::from(&event)), session.shelf(), DarkSwitch::new(session))} }
    }

    fn landing(&self, key: Visit) -> Element {
        rsx! {
            {self.session.landing_top().into_iter().map(|top| rsx! {
                div { key: "{key}", class: "scroll-home", "aria-hidden": "true",
                    onmounted: move |_| {
                        let jumper = document::eval(Self::JUMP_TOP);
                        jumper.send(AsRef::<Pixels>::as_ref(&top).pixels()).unwrap_or_default();
                    },
                }
            })}
        }
    }
}

impl IntoDynNode for ScreenFrame<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let session = self.session;
        let screen = self.screen;
        let probe = session.clone();
        let reader = session.clone();
        let root = RootStyle::new(session.viewport());
        let shift: PageShift = session.page_shift();
        let page = rsx! {
            div { class: Look::screen(&screen).class(), style: "{shift}",
                {screen.wiki_entry().is_none().then(|| BackChip::new(session))}
                {self.body()}
            }
        };
        rsx! {
            style { "{root}" }
            div { class: session.appearance().class(), "aria-hidden": "true" }
            {self.landing(session.visit())}
            div { class: "viewport-probe", "aria-hidden": "true",
                onresize: move |event| {
                    if let Ok(size) = event.get_border_box_size() {
                        probe.measure_viewport(|viewport| viewport.at_width(Pixels::new(size.width)));
                    }
                },
            }
            span { class: "type-probe", "aria-hidden": "true",
                onresize: move |event| {
                    if let Ok(size) = event.get_border_box_size() {
                        reader.measure_viewport(|viewport| {
                            viewport.read_at(Viewport::reading_of(Pixels::new(size.height)))
                        });
                    }
                },
                "M"
            }
            {TouchSurface::Page.wrap(session, page)}
            {(session.menu() != Menu::CLOSED).then(|| MenuView::new(session))}
            {TouchSurface::tabs(session, rsx! {
                {Screen::TABS.iter().map(|tab| {
                    let target = *tab;
                    let session = session.clone();
                    rsx! {
                        button {
                            class: Look::tab(tab, &screen).class(),
                            onclick: move |event| SessionEvent::OpenTab { screen: target, top: ScrollTop::from(&event) }.applied(&session),
                            "{tab.tab_label()}"
                        }
                    }
                })}
            })}
        }
        .into_dyn_node()
    }
}
