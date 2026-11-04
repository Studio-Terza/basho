use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::guidance::Phase;
use guide::readout::Status;

mod status_look;

use crate::app::screen::map_view::narration::Narration;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::time_view::TimeView;
use status_look::StatusLook;

#[derive(newtype::Newtype)]
pub struct StatusText<'view> {
    narration: Narration<'view>,
}

impl StatusText<'_> {
    fn status_words(status: Status) -> Element {
        status.leaves().map_or_else(
            || rsx! { "{status}" },
            |until| rsx! { "WAITING · leaves " {TimeView::new(until)} },
        )
    }
}

impl IntoDynNode for StatusText<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let guidance = self.narration.guidance();
        let language = self.narration.language();
        let quiet = Some(guidance.phase())
            .filter(|phase| *phase == Phase::Preview)
            .map_or("DAY DONE", |_| "NOT STARTED");
        guidance.stop().filter(|_| guidance.status().is_some()).map_or_else(
            || {
                guidance.status().map_or_else(
                    || rsx! { p { class: "status", "{quiet}" } },
                    |status| rsx! {
                        p { class: StatusLook::for_emphasis(status.emphasis()).class(), {Self::status_words(status)} }
                    },
                )
            },
            |stop| rsx! {
                p { class: "status stopped", lang: language.class(),
                    "STOPPED · " {RubyView::new(stop.planned().name())}
                    span { class: "stop-time", " · {stop.spent().count()} of {stop.planned().booked().count()} min" }
                }
            },
        )
        .into_dyn_node()
    }
}
