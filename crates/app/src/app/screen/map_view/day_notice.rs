use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::FactSource;
use guide::classed::Classed;
use guide::clock::DayTense;
use guide::end::End;
use guide::leg_line::LegLine;

use crate::app::screen::map_view::narration::Narration;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::time_view::TimeView;

#[derive(newtype::Newtype)]
pub struct DayNotice<'view> {
    narration: Narration<'view>,
}

impl DayNotice<'_> {
    fn shown(&self, tense: DayTense) -> Element {
        match tense {
            DayTense::Today => rsx! {},
            DayTense::Past => rsx! {
                section { class: "buffer day-notice", role: "group", "aria-label": "Day",
                    p { class: "buffer-line", "Ended " {self.narration.guidance().day().date().short().to_string()} }
                }
            },
            DayTense::Ahead => rsx! {
                section { class: "buffer day-notice", role: "group", "aria-label": "Day start",
                    p { class: "buffer-line", lang: self.narration.language().class(),
                        "Starts " {self.narration.guidance().day().date().short().to_string()}
                        {self.narration.guidance().day().landing().map(|landing| {
                            rsx! { " · " {RubyView::new(landing.flight().as_ref())} " " {TimeView::worded(landing.lands(), FactSource::Lands.word().lowered())} }
                        })}
                        {self.narration.guidance().day().legs().first().map(|leg| {
                            rsx! {
                                " · first: " {TimeView::worded(leg.listed(End::Start), leg.mode().label().lowered())} ", " {RubyView::new(leg.line_text(LegLine::Way))}
                            }
                        })}
                    }
                }
            },
        }
    }
}

impl IntoDynNode for DayNotice<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        self.shown(self.narration.guidance().tense())
            .into_dyn_node()
    }
}
