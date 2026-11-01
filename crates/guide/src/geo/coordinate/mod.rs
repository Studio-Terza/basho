mod latitude;
mod longitude;

use serde::Deserialize;

use crate::geo::bearing::Bearing;
use crate::geo::degrees::Degrees;
use crate::geo::distance::Distance;
use crate::geo::share::Share;

pub use latitude::Latitude;
pub use longitude::Longitude;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Coordinate {
    lat: Latitude,
    lon: Longitude,
}

impl Coordinate {
    const EARTH_RADIUS: Distance = Distance::new(6_371_008.8);
    const EARTH_DIAMETER: Distance = Distance::new(2.0 * 6_371_008.8);
    pub const LON_LAT_LENGTH: usize = 2;

    #[must_use]
    pub const fn new(lat: Degrees, lon: Degrees) -> Coordinate {
        Coordinate {
            lat: Latitude::new(lat),
            lon: Longitude::new(lon),
        }
    }

    #[must_use]
    pub const fn lat(self) -> Latitude {
        self.lat
    }

    #[must_use]
    pub const fn lon(self) -> Longitude {
        self.lon
    }

    #[must_use]
    pub fn distance_to(self, other: Coordinate) -> Distance {
        let from = self.lat.angle().degrees().to_radians();
        let to = other.lat.angle().degrees().to_radians();
        let lat_sine = ((other.lat.angle() - self.lat.angle())
            .degrees()
            .to_radians()
            * Share::HALF.fraction())
        .sin();
        let lon_sine = ((other.lon.angle() - self.lon.angle())
            .degrees()
            .to_radians()
            * Share::HALF.fraction())
        .sin();
        let chord = lat_sine * lat_sine + from.cos() * to.cos() * (lon_sine * lon_sine);
        Self::EARTH_DIAMETER * chord.sqrt().asin()
    }

    #[must_use]
    pub fn bearing_to(self, other: Coordinate) -> Bearing {
        let delta_lon = (other.lon.angle() - self.lon.angle())
            .degrees()
            .to_radians();
        let from = self.lat.angle().degrees().to_radians();
        let to = other.lat.angle().degrees().to_radians();
        let east = delta_lon.sin() * to.cos();
        let north = from.cos() * to.sin() - from.sin() * to.cos() * delta_lon.cos();
        Bearing::new(east.atan2(north).to_degrees())
    }

    #[must_use]
    pub fn offset_by(self, east: Distance, north: Distance) -> Coordinate {
        let parallel = Self::EARTH_RADIUS * self.lat.angle().degrees().to_radians().cos();
        Coordinate::new(
            self.lat.angle() + Degrees::new((north / Self::EARTH_RADIUS).to_degrees()),
            self.lon.angle() + Degrees::new((east / parallel).to_degrees()),
        )
    }

    #[must_use]
    pub fn toward(self, other: Coordinate, share: Share) -> Coordinate {
        Coordinate::new(
            self.lat.angle() + (other.lat.angle() - self.lat.angle()).scaled(share.fraction()),
            self.lon.angle() + (other.lon.angle() - self.lon.angle()).scaled(share.fraction()),
        )
    }
}

impl<Holder: AsRef<Coordinate>> From<&Holder> for Coordinate {
    fn from(holder: &Holder) -> Coordinate {
        *holder.as_ref()
    }
}
