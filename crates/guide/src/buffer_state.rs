use crate::clock::Minutes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferState {
    Comfortable,
    Tight,
    Short,
    Waiting,
}

impl BufferState {
    pub const TIGHT_BELOW: Minutes = Minutes::new(10);

    #[must_use]
    pub fn for_spare(spare: Minutes) -> BufferState {
        if spare < Minutes::ZERO {
            BufferState::Short
        } else if spare < Self::TIGHT_BELOW {
            BufferState::Tight
        } else {
            BufferState::Comfortable
        }
    }
}
