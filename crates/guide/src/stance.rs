#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stance {
    Preview,
    Waiting,
    Moving,
    Missed,
    Done,
}

impl Stance {
    #[must_use]
    pub fn underway(self) -> Option<Stance> {
        match self {
            Stance::Waiting | Stance::Moving | Stance::Missed => Some(self),
            Stance::Preview | Stance::Done => None,
        }
    }
}
