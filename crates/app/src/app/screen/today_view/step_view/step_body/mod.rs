mod arrival_row;
mod briefing;
mod pin_button;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::Language;
use guide::guidance::Guidance;
use guide::leg::Leg;

use crate::app::session::Session;
use crate::app::step::{Step, StepPart, StepState};
use arrival_row::ArrivalRow;
use briefing::Briefing;
use pin_button::PinButton;

pub struct StepBody<'view> {
    session: &'view Session,
    guidance: &'view Guidance<'view>,
    step: &'view Step<'view>,
    state: StepState,
    leg: &'view Leg,
    language: Language,
}

impl<'view> StepBody<'view> {
    pub fn new(
        session: &'view Session,
        guidance: &'view Guidance<'view>,
        step: &'view Step<'view>,
        state: StepState,
    ) -> Self {
        StepBody {
            session,
            guidance,
            step,
            state,
            leg: step.leg(),
            language: session.language(),
        }
    }

    fn rows(&self) -> Element {
        match self.step.part() {
            StepPart::Walk => rsx! {
                dl { class: "rows",
                    {Briefing::Why.shown(self)}
                    {Briefing::Route.shown(self)}
                    {Briefing::How.shown(self)}
                    {ArrivalRow::OnTheWay.shown(self)}
                    {ArrivalRow::There.shown(self)}
                    {ArrivalRow::Warnings.shown(self)}
                    {Briefing::Ifs.shown(self)}
                    {Briefing::Asks.shown(self)}
                }
                {ArrivalRow::PlaceLink.shown(self)}
                {PinButton::new(self.session, self.guidance.day(), self.step, self.state)}
            },
            StepPart::Board => rsx! {
                dl { class: "rows",
                    {Briefing::Train.shown(self)}
                    {Briefing::Why.shown(self)}
                    {Briefing::How.shown(self)}
                    {Briefing::Ifs.shown(self)}
                    {Briefing::Asks.shown(self)}
                }
                {ArrivalRow::PlaceLink.shown(self)}
                {PinButton::new(self.session, self.guidance.day(), self.step, self.state)}
            },
            StepPart::Alight => rsx! {
                dl { class: "rows",
                    {ArrivalRow::OnTheWay.shown(self)}
                    {ArrivalRow::There.shown(self)}
                    {ArrivalRow::Warnings.shown(self)}
                }
                {PinButton::new(self.session, self.guidance.day(), self.step, self.state)}
            },
        }
    }
}

impl IntoDynNode for StepBody<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        self.rows().into_dyn_node()
    }
}
