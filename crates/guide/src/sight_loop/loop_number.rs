use std::fmt;
use std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoopNumber(NonZeroUsize);

impl LoopNumber {
    #[must_use]
    pub fn after(index: usize) -> LoopNumber {
        LoopNumber(NonZeroUsize::MIN.saturating_add(index))
    }
}

impl fmt::Display for LoopNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
