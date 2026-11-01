use std::fmt;

use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct Elevation(u32);

impl Elevation {
    #[must_use]
    pub const fn meters(self) -> u32 {
        self.0
    }
}

impl fmt::Display for Elevation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} m", self.0)
    }
}
