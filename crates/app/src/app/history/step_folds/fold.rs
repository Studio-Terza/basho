#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fold {
    Open,
    Closed,
}

impl Fold {
    #[must_use]
    pub fn flipped(self) -> Fold {
        match self {
            Fold::Open => Fold::Closed,
            Fold::Closed => Fold::Open,
        }
    }
}
