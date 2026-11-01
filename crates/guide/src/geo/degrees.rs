use newtype::Newtype;

use serde::Deserialize;

use crate::geo::not_finite::NotFinite;

#[derive(Newtype, Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize)]
#[serde(try_from = "f64")]
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
    #[must_use]
    pub const fn degrees(self) -> f64 {
        self.0
    }

    #[must_use]
    pub fn scaled(self, factor: f64) -> Degrees {
        Degrees(self.0 * factor)
    }
}
