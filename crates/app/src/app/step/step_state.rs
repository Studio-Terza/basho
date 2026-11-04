use guide::classed::Classed;
use std::cmp::Ordering;
use std::fmt;

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepState {
    #[strum(serialize = "done")]
    Done,
    #[strum(serialize = "current")]
    Current,
    #[strum(serialize = "later")]
    Later,
}

impl Classed for StepState {}

impl fmt::Display for StepState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepState::Done => formatter.write_str("✓"),
            StepState::Current => formatter.write_str("▶"),
            StepState::Later => formatter.write_str(""),
        }
    }
}

impl From<Ordering> for StepState {
    fn from(step_to_current: Ordering) -> StepState {
        match step_to_current {
            Ordering::Less => StepState::Done,
            Ordering::Equal => StepState::Current,
            Ordering::Greater => StepState::Later,
        }
    }
}
