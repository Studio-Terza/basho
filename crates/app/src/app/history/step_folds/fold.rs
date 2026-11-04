use crate::app::step::StepState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fold {
    Open,
    Closed,
}

impl From<StepState> for Fold {
    fn from(state: StepState) -> Fold {
        match state {
            StepState::Current => Fold::Open,
            StepState::Done | StepState::Later => Fold::Closed,
        }
    }
}

impl Fold {
    #[must_use]
    pub fn flipped(self) -> Fold {
        match self {
            Fold::Open => Fold::Closed,
            Fold::Closed => Fold::Open,
        }
    }
}
