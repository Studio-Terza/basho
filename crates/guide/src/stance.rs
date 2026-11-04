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
    pub fn playing(self) -> Option<Stance> {
        match self {
            Stance::Preview | Stance::Done => None,
            Stance::Waiting | Stance::Moving | Stance::Missed => Some(self),
        }
    }
}
