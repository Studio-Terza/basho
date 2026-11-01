use std::fmt;

use crate::geo::Distance;

#[derive(derive_more::Into, Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Remaining(Distance);

impl Remaining {
    #[must_use]
    pub fn between(along: Distance, length: Distance) -> Remaining {
        Remaining((length - along).max(Distance::default()))
    }
}

impl fmt::Display for Remaining {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
