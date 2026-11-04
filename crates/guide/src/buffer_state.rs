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
    const BELOW: [(Minutes, BufferState); 2] = [
        (Minutes::ZERO, BufferState::Short),
        (Self::TIGHT_BELOW, BufferState::Tight),
    ];

    #[must_use]
    pub fn for_spare(spare: Minutes) -> BufferState {
        Self::BELOW
            .into_iter()
            .find(|(limit, _)| spare < *limit)
            .map_or(BufferState::Comfortable, |(_, state)| state)
    }
}
