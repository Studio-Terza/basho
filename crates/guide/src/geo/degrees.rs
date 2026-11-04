use newtype::Newtype;

use serde::Deserialize;

use crate::geo::not_finite::NotFinite;

#[derive(
    Newtype, newtype::Arbitrary, Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize,
)]
#[serde(try_from = "f64")]
#[newtype(strategy = -Self::LARGEST_DRAWN..Self::LARGEST_DRAWN)]
pub struct Degrees(f64);

added!(Degrees);
subtracted!(Degrees);

impl TryFrom<f64> for Degrees {
    type Error = NotFinite;

    fn try_from(number: f64) -> Result<Degrees, NotFinite> {
        NotFinite::ensure_finite(number).map(Degrees)
    }
}

impl Degrees {
    const LARGEST_DRAWN: f64 = 1.0e7;

    #[must_use]
    pub const fn degrees(self) -> f64 {
        self.0
    }

    #[must_use]
    pub fn scaled(self, factor: f64) -> Degrees {
        Degrees(self.0 * factor)
    }
}
