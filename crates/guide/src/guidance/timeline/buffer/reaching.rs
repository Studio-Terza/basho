use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reaching {
    InTime,
    Late,
}

impl From<Ordering> for Reaching {
    fn from(against_limit: Ordering) -> Reaching {
        match against_limit {
            Ordering::Less | Ordering::Equal => Reaching::InTime,
            Ordering::Greater => Reaching::Late,
        }
    }
}

impl Reaching {
    #[must_use]
    pub fn keep<T>(self, found: T) -> Option<T> {
        match self {
            Reaching::InTime => Some(found),
            Reaching::Late => None,
        }
    }
}
