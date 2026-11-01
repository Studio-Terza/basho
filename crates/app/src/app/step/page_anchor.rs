use std::fmt;

use crate::app::step::{GoalNumber, StepNumber};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageAnchor {
    Goal(GoalNumber),
    Step(StepNumber),
}

impl fmt::Display for PageAnchor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PageAnchor::Goal(number) => write!(formatter, "goal-{number}"),
            PageAnchor::Step(number) => write!(formatter, "step-{number}"),
        }
    }
}
