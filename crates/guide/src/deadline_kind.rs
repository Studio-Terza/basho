#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeadlineKind {
    Ride,
    Tour,
    Closes,
    Entry,
    LastOrder,
    CheckIn,
}
