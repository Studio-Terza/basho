#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeadlineKind {
    Ride,
    Tour,
    Closes,
    Entry,
    LastOrder,
    CheckIn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StretchEnd {
    Ends,
    Continues,
}

impl DeadlineKind {
    #[must_use]
    pub fn ending(self) -> StretchEnd {
        match self {
            DeadlineKind::Ride | DeadlineKind::Tour | DeadlineKind::CheckIn => StretchEnd::Ends,
            DeadlineKind::Closes | DeadlineKind::Entry | DeadlineKind::LastOrder => {
                StretchEnd::Continues
            }
        }
    }
}
