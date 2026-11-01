use newtype::Newtype;
use std::fmt;

use serde::Deserialize;

use crate::geo::not_finite::NotFinite;

#[derive(Newtype, Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize)]
#[serde(try_from = "f64")]
pub struct Kilometers(f64);

impl TryFrom<f64> for Kilometers {
    type Error = NotFinite;

    fn try_from(number: f64) -> Result<Kilometers, NotFinite> {
        NotFinite::ensure_finite(number).map(Kilometers)
    }
}

impl Kilometers {
    #[must_use]
    pub const fn amount(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Kilometers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}
