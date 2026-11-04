use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::itinerary::Day;

use crate::app::session::{Session, SessionEvent};
use crate::app::step::{Spot, Step, StepState};

pub struct PinButton<'view> {
    session: &'view Session,
    day: &'view Day,
    step: &'view Step<'view>,
    state: StepState,
}

impl<'view> PinButton<'view> {
    pub fn new(
        session: &'view Session,
        day: &'view Day,
        step: &'view Step<'view>,
        state: StepState,
    ) -> Self {
        PinButton {
            session,
            day,
            step,
            state,
        }
    }
}

impl PinButton<'_> {
    fn spot(&self) -> Spot {
        Spot::new(self.day.number(), self.step.number())
    }

    fn shown(&self) -> Element {
        match (self.session.now().trip_date() == self.day.date()).then_some(self.state) {
            None => rsx! {},
            Some(StepState::Current) => rsx! {
                button { class: "step-set", onclick: {
                    let session = self.session.clone();
                    let spot = self.spot();
                    move |_| SessionEvent::MarkDone(spot).applied(&session)
                }, "Done ›" }
            },
            Some(StepState::Done | StepState::Later) => rsx! {
                button { class: "step-set", onclick: {
                    let session = self.session.clone();
                    let spot = self.spot();
                    move |_| SessionEvent::PlaceHere(spot).applied(&session)
                }, "I'm here" }
            },
        }
    }
}

impl IntoDynNode for PinButton<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        self.shown().into_dyn_node()
    }
}
