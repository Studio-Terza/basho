mod fold;

use guide::day_number::DayNumber;

use crate::app::step::{StepNumber, StepState};
pub use fold::Fold;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StepFolds {
    day: Option<DayNumber>,
    flipped: Vec<StepNumber>,
}

impl StepFolds {
    #[must_use]
    pub fn fold(&self, day: DayNumber, step: StepNumber, state: StepState) -> Fold {
        let usual = match state {
            StepState::Current => Fold::Open,
            StepState::Done | StepState::Later => Fold::Closed,
        };
        if self.day == Some(day) && self.flipped.contains(&step) {
            usual.flipped()
        } else {
            usual
        }
    }

    #[must_use]
    pub fn toggled(&self, day: DayNumber, step: StepNumber) -> StepFolds {
        let kept = self
            .flipped
            .iter()
            .copied()
            .filter(|_| self.day == Some(day))
            .filter(|flipped| *flipped != step);
        let added = (!self.flipped.contains(&step) || self.day != Some(day)).then_some(step);
        StepFolds {
            day: Some(day),
            flipped: kept.chain(added).collect(),
        }
    }
}
