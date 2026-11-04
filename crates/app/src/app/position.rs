#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position(usize);

impl Position {
    #[must_use]
    pub const fn new(position: usize) -> Position {
        Position(position)
    }

    #[must_use]
    pub const fn into_inner(self) -> usize {
        self.0
    }
}
