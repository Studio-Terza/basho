mod map_key_view;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::words::Label;
use map_key_view::MapKeyView;
use map_plan::{MapLevel, MapMode};
use newtype::Newtype;

use crate::app::session::{MapHealth, Session, SessionEvent};

#[derive(Newtype)]
pub struct DayMapView<'view> {
    session: &'view Session,
}

impl DayMapView<'_> {
    fn level_words(level: MapLevel) -> Label {
        match level {
            MapLevel::Leg => Label::new("Leg"),
            MapLevel::Goal => Label::new("Goal"),
            MapLevel::Day => Label::new("Whole day"),
        }
    }

    fn mode_words(mode: MapMode) -> Label {
        match mode {
            MapMode::Guide => Label::new("Explore"),
            MapMode::Explore => Label::new("Guide"),
        }
    }
}

impl IntoDynNode for DayMapView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let session = self.session.clone();
        let mode = self.session.map_mode();
        let level = self.session.map_level();
        rsx! {
            div { class: "day-map",
                div { class: "map-slot", role: "img", "aria-label": "Map of this step, with you and the place at its end" }
                {(self.session.map_health() == MapHealth::Failed).then(|| rsx! {
                    p { class: "map-failed", role: "alert", "The map could not start. The steps still work." }
                })}
                div { class: "map-controls", "data-overlay": "true",
                    {(mode == MapMode::Guide).then(|| rsx! { span { class: "map-level", {Self::level_words(level).to_string()} } })}
                    button { class: "map-mode",
                        onclick: move |_| SessionEvent::SwitchMapMode.applied(&session),
                        {Self::mode_words(mode).to_string()}
                    }
                }
                {MapKeyView::new(self.session)}
            }
        }
        .into_dyn_node()
    }
}
