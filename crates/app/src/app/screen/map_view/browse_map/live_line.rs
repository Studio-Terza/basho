use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;

use crate::app::screen::buffer_view::{BufferShape, BufferView};
use crate::app::screen::map_view::narration::Narration;

#[derive(newtype::Newtype)]
pub struct LiveLine<'view> {
    narration: Narration<'view>,
}

impl IntoDynNode for LiveLine<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let language = self.narration.language();
        let late = self
            .narration
            .guidance()
            .time_check()
            .filter(|check| check.late_by().is_some());
        let buffer = self.narration.guidance().buffer();
        rsx! {
            {(late.is_some() || buffer.is_some()).then(|| rsx! {
                p { class: "live-line", role: "status", lang: language.class(), "aria-label": "Now, for where you are",
                    strong { class: "live-now", "NOW" }
                    {late.map(|check| rsx! { " · " span { class: "late", "{check}" } })}
                    {buffer.map(|buffer| rsx! { " · " {BufferView::new(&buffer, language).render(BufferShape::Live)} })}
                }
            })}
        }
        .into_dyn_node()
    }
}
