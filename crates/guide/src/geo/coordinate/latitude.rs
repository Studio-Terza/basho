use newtype::Newtype;
use newtype::proptest::strategy::Strategy;
use serde::Deserialize;

use crate::geo::coordinate::Coordinate;
use crate::geo::degrees::Degrees;
use crate::geo::distance::Distance;

#[derive(Newtype, newtype::Arbitrary, Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize)]
#[serde(transparent)]
#[newtype(strategy = (-Self::FURTHEST..=Self::FURTHEST).prop_map(Degrees::new))]
pub struct Latitude(Degrees);

impl Latitude {
    const FURTHEST: f64 = 90.0;

    #[must_use]
    pub const fn angle(self) -> Degrees {
        self.0
    }

    #[must_use]
    pub fn north_of(self, origin: Coordinate) -> Distance {
        Coordinate::EARTH_RADIUS * (self.0 - origin.lat().0).degrees().to_radians()
    }
}
