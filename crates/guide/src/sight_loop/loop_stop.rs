use super::{LoopNumber, LoopSight};

#[derive(derive_more::AsRef, Clone, Copy, Debug)]
pub struct LoopStop<'sight> {
    number: LoopNumber,
    #[as_ref(forward)]
    sight: &'sight LoopSight,
}

impl<'sight> LoopStop<'sight> {
    #[must_use]
    pub fn new(number: LoopNumber, sight: &'sight LoopSight) -> LoopStop<'sight> {
        LoopStop { number, sight }
    }

    #[must_use]
    pub fn number(&self) -> LoopNumber {
        self.number
    }
}
