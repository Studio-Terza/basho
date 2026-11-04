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
        let usual = Fold::from(state);
        self.flipped_at(day, step)
            .map_or(usual, |_| usual.flipped())
    }

    #[must_use]
    pub fn toggled(&self, day: DayNumber, step: StepNumber) -> StepFolds {
        let kept = self
            .flipped
            .iter()
            .copied()
            .filter(|_| self.day == Some(day))
            .filter(|flipped| *flipped != step);
        let added = self.flipped_at(day, step).map_or(Some(step), |_| None);
        StepFolds {
            day: Some(day),
            flipped: kept.chain(added).collect(),
        }
    }

    fn flipped_at(&self, day: DayNumber, step: StepNumber) -> Option<&StepNumber> {
        Some(day)
            .filter(|day| self.day == Some(*day))
            .and_then(|_| self.flipped.iter().find(|flipped| **flipped == step))
    }
}
