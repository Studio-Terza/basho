use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Before,
    After,
}

impl From<Ordering> for Side {
    fn from(against_finish: Ordering) -> Side {
        match against_finish {
            Ordering::Less | Ordering::Equal => Side::Before,
            Ordering::Greater => Side::After,
        }
    }
}

impl Side {
    #[must_use]
    pub fn nearest<T>(self, before: Option<T>, after: Option<T>) -> Option<T> {
        match self {
            Side::Before => before.or(after),
            Side::After => after.or(before),
        }
    }
}
