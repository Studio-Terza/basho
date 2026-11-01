#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reach {
    Open,
    End,
}

impl Reach {
    pub fn of_target<Target>(target: Option<Target>) -> Reach {
        target.map_or(Reach::End, |_| Reach::Open)
    }

    pub fn flag(self) -> Option<bool> {
        match self {
            Reach::Open => None,
            Reach::End => Some(true),
        }
    }
}
