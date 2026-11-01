mod step_body;

use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::classed::Classed;
use guide::clock::Minutes;
use guide::guidance::Guidance;
use guide::itinerary::Day;

use crate::app::history::Fold;
use crate::app::screen::look::Look;
use crate::app::screen::scroll_anchor::ScrollAnchor;
use crate::app::screen::step_title::StepTitle;
use crate::app::screen::time_view::TimeView;
use crate::app::session::Session;
use crate::app::step::{Spot, Step, StepState};
use step_body::StepBody;

pub struct StepView<'view> {
    session: &'view Session,
    guidance: &'view Guidance<'view>,
    step: &'view Step<'view>,
    state: StepState,
}

impl<'view> StepView<'view> {
    pub fn new(
        session: &'view Session,
        guidance: &'view Guidance<'view>,
        step: &'view Step<'view>,
        state: StepState,
    ) -> Self {
        StepView {
            session,
            guidance,
            step,
            state,
        }
    }
}

impl IntoDynNode for StepView<'_> {
    fn into_dyn_node(self) -> DynamicNode {
        let language = self.session.language();
        let step = self.step;
        let day: &Day = self.guidance.day();
        let number = step.number();
        let fold = self.session.fold(day.number(), number, self.state);
        let toggler = self.session.clone();
        let day_number = day.number();
        let planned = step.planned();
        let visit = self.session.visit();
        let current = visit
            .landing()
            .step((self.state == StepState::Current).then_some(number))
            .filter(|target| *target == number);
        let focus = Look::step_focus(
            (visit.focus().spot() == Some(Spot::new(day_number, number))).then_some(()),
        );
        rsx! {
            li { id: "{number.anchor()}", class: "step {self.state.class()}{focus.class()}",
                {current.map(|_| ScrollAnchor::new(visit))}
                button { class: "step-head", "aria-expanded": "{fold == Fold::Open}",
                    onclick: move |_| toggler.toggle_step(day_number, number),
                    span { class: "step-mark", "aria-hidden": "true", "{self.state}" }
                    span { class: "step-n", "{number}" }
                    span { class: "step-title", lang: language.class(), {StepTitle::new(step)} }
                    span { class: "step-time",
                        {TimeView::new(self.guidance.shifted(step.index(), step.due()))}
                        {(planned > Minutes::ZERO).then(|| rsx! { " · {planned}" })}
                    }
                }
                {(fold == Fold::Open).then(|| StepBody::new(self.session, self.guidance, step, self.state))}
            }
        }
        .into_dyn_node()
    }
}
