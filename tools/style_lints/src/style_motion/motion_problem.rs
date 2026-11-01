use std::fmt;

use crate::style_sheet::{CssClass, CssValue, Declaration, Selector};

pub enum MotionProblem<'rule> {
    Keyframes(&'rule Selector),
    Animated(&'rule Selector, &'rule Declaration),
    MovedFromOutside(&'rule Selector, CssClass),
    LiteralPose(&'rule Selector, &'rule Declaration),
    Unfollowed(&'rule Selector, &'rule CssValue),
}

impl fmt::Display for MotionProblem<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MotionProblem::Keyframes(selector) => write!(
                formatter,
                "{selector} replays whenever its animation is reapplied"
            ),
            MotionProblem::Animated(selector, declaration) => {
                write!(formatter, "{selector} sets {}", declaration.property())
            }
            MotionProblem::MovedFromOutside(selector, class) => write!(
                formatter,
                "{selector} moves .{class} from outside its own rule"
            ),
            MotionProblem::LiteralPose(selector, declaration) => write!(
                formatter,
                "{selector} {}: {} is not a pose var",
                declaration.property(),
                declaration.assigned()
            ),
            MotionProblem::Unfollowed(selector, value) => write!(
                formatter,
                "{selector} transition: {value} does not take its time from the pose"
            ),
        }
    }
}
