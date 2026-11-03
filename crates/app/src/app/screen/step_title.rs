use dioxus::dioxus_core::DynamicNode;
use newtype::Newtype;
use std::fmt;

use dioxus::prelude::*;
use guide::end::End;
use guide::leg_line::LegLine;
use guide::reading::Reading;
use guide::ruby_text::RubyText;
use guide::words::Spelling;

use crate::app::screen::ruby_view::RubyView;
use crate::app::screen::time_view::TimeView;
use crate::app::step::{Step, StepPart};

#[derive(Newtype)]
pub struct StepTitle<'step> {
    step: &'step Step<'step>,
}

impl<'step> StepTitle<'step> {
    pub fn place_words(&self) -> Spelling {
        self.place().text(Reading::Spoken)
    }

    pub fn place(&self) -> &'step RubyText {
        self.step.leg().line_text(LegLine::Reach)
    }
}

impl IntoDynNode for StepTitle<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let leg = self.step.leg();
        let place = RubyView::new(self.place());
        match self.step.part() {
            StepPart::Walk => rsx! { "Get to " {place} },
            StepPart::Board => rsx! {
                "Get on the " {TimeView::worded(leg.listed(End::Start), leg.mode().label().lowered())} " to " {place}
            },
            StepPart::Alight => rsx! { "Get off at " {place} },
        }
        .into_dyn_node()
    }
}

impl fmt::Display for StepTitle<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let leg = self.step.leg();
        let place = self.place_words();
        match self.step.part() {
            StepPart::Walk => write!(formatter, "Get to {place}"),
            StepPart::Board => write!(
                formatter,
                "Get on the {} to {place}",
                leg.listed(End::Start).worded(leg.mode().label().lowered())
            ),
            StepPart::Alight => write!(formatter, "Get off at {place}"),
        }
    }
}
