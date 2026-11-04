use guide::geo::{Coordinate, Degrees};
use serde::ser::{Serialize, Serializer};

use crate::own_layer::GeoPoint;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapBounds {
    southwest: GeoPoint,
    northeast: GeoPoint,
}

impl MapBounds {
    #[must_use]
    pub fn around(points: &[Coordinate]) -> Option<MapBounds> {
        let lons = || points.iter().map(|point| point.lon().angle());
        let lats = || points.iter().map(|point| point.lat().angle());
        let by = |left: &Degrees, right: &Degrees| left.degrees().total_cmp(&right.degrees());
        let (west, south) = lons().min_by(by).zip(lats().min_by(by))?;
        let (east, north) = lons().max_by(by).zip(lats().max_by(by))?;
        Some(MapBounds {
            southwest: GeoPoint::from(Coordinate::new(south, west)),
            northeast: GeoPoint::from(Coordinate::new(north, east)),
        })
    }
}

impl Serialize for MapBounds {
    fn serialize<Target: Serializer>(&self, target: Target) -> Result<Target::Ok, Target::Error> {
        (self.southwest, self.northeast).serialize(target)
    }
}
