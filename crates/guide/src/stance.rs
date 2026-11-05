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
    pub fn playing<T>(self, found: Option<T>) -> Option<T> {
        match self {
            Stance::Preview | Stance::Done => None,
            Stance::Waiting | Stance::Moving | Stance::Missed => found,
        }
    }
}
