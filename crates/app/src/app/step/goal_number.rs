use crate::app::step::PageAnchor;

#[derive(newtype::Display, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GoalNumber(usize);

impl GoalNumber {
    #[must_use]
    pub const fn new(position: usize) -> GoalNumber {
        GoalNumber(position + 1)
    }

    #[must_use]
    pub fn anchor(self) -> PageAnchor {
        PageAnchor::Goal(self)
    }
}

impl From<GoalNumber> for usize {
    fn from(goal_number: GoalNumber) -> usize {
        goal_number.0.saturating_sub(1)
    }
}
