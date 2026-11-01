use guide::classed::Classed;
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
        formatter.write_str(match self {
            StepState::Done => "✓",
            StepState::Current => "▶",
            StepState::Later => "",
        })
    }
}
