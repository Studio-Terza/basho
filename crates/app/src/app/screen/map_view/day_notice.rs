use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::clock::DayTense;
use guide::end::End;
use guide::leg_line::LegLine;
use guide::times::FactSource;

use crate::app::screen::map_view::narration::Narration;
use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::time_view::TimeView;

#[derive(newtype::Newtype)]
pub struct DayNotice<'view> {
    narration: Narration<'view>,
}

impl IntoDynNode for DayNotice<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let language = self.narration.language();
        let day = self.narration.guidance().day();
        let date = day.date().short();
        match self.narration.guidance().tense() {
            DayTense::Today => rsx! {},
            DayTense::Past => rsx! {
                section { class: "buffer day-notice", role: "group", "aria-label": "Day",
                    p { class: "buffer-line", "Ended {date}" }
                }
            },
            DayTense::Ahead => rsx! {
                section { class: "buffer day-notice", role: "group", "aria-label": "Day start",
                    p { class: "buffer-line", lang: language.class(),
                        "Starts {date}"
                        {day.landing().map(|landing| {
                            rsx! { " · " {RubyView::new(landing.flight().as_ref())} " " {TimeView::worded(landing.lands(), FactSource::Lands.word().lowered())} }
                        })}
                        {day.legs().first().map(|leg| {
                            rsx! {
                                " · first: " {TimeView::worded(leg.listed(End::Start), leg.mode().label().lowered())} ", " {RubyView::new(leg.line_text(LegLine::Way))}
                            }
                        })}
                    }
                }
            },
        }
        .into_dyn_node()
    }
}
