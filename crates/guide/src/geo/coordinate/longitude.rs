use newtype::Newtype;
use newtype::proptest::strategy::Strategy;
use serde::Deserialize;

use crate::geo::coordinate::Coordinate;
use crate::geo::degrees::Degrees;
use crate::geo::distance::Distance;
use crate::geo::share::Share;

#[derive(Newtype, newtype::Arbitrary, Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize)]
#[serde(transparent)]
#[newtype(strategy = (-Self::FURTHEST..=Self::FURTHEST).prop_map(Degrees::new))]
pub struct Longitude(Degrees);

impl Longitude {
    const FURTHEST: f64 = 180.0;

    #[must_use]
    pub const fn angle(self) -> Degrees {
        self.0
    }

    #[must_use]
    pub fn east_of(self, origin: Coordinate) -> Distance {
        self.east_across(
            origin,
            Share::new(origin.lat().angle().degrees().to_radians().cos()),
        )
    }

    #[must_use]
    pub fn east_across(self, origin: Coordinate, parallel: Share) -> Distance {
        Coordinate::EARTH_RADIUS
            * ((self.0 - origin.lon().0).degrees().to_radians() * parallel.fraction())
    }
}
