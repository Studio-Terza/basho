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
mod unverified_mark;
mod wiki_view;
pub mod wording;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::english_name::EnglishName;
use guide::map_frame::Pixels;
use layout::RootStyle;
use layout::scale::Viewport;
use map_plan::{MapSent, MapUpdate};

use crate::app::history::ScrollTop;
use crate::app::session::{PageShift, Session, SessionEvent};
use crate::app::visit::{Pane, Screen, Visit};
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
use wiki_view::WikiView;

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
            || match self.screen.pane() {
                Pane::Map => rsx! { {MapView::new(session)} },
                Pane::Today => rsx! { {TodayView::new(session)} },
                Pane::Trip => Self::trip_page(session),
                Pane::Reports => rsx! { {ReportsView::new(session)} },
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
                            "{tab.tab()}"
                        }
                    }
                })}
            })}
        }
        .into_dyn_node()
    }
}
