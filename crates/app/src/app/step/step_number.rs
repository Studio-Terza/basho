use crate::app::step::PageAnchor;

#[derive(newtype::Display, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct StepNumber(usize);

impl StepNumber {
    #[must_use]
    pub const fn from_position(position: usize) -> StepNumber {
        StepNumber(position + 1)
    }

    #[must_use]
    pub fn anchor(self) -> PageAnchor {
        PageAnchor::Step(self)
    }
}

impl From<StepNumber> for usize {
    fn from(step_number: StepNumber) -> usize {
        step_number.0.saturating_sub(1)
    }
}
