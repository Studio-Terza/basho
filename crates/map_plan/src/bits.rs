use guide::geo::{Bearing, Coordinate};
use serde::ser::{Serialize, SerializeTuple, Serializer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeoPoint(Coordinate);

impl From<Coordinate> for GeoPoint {
    fn from(point: Coordinate) -> GeoPoint {
        GeoPoint(point)
    }
}

impl Serialize for GeoPoint {
    fn serialize<Target: Serializer>(&self, target: Target) -> Result<Target::Ok, Target::Error> {
        let mut pair = target.serialize_tuple(2)?;
        pair.serialize_element(&self.0.lon().angle().degrees())?;
        pair.serialize_element(&self.0.lat().angle().degrees())?;
        pair.end()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeoHeading(Bearing);

impl From<Bearing> for GeoHeading {
    fn from(facing: Bearing) -> GeoHeading {
        GeoHeading(facing)
    }
}

impl Serialize for GeoHeading {
    fn serialize<Target: Serializer>(&self, target: Target) -> Result<Target::Ok, Target::Error> {
        target.serialize_f64(self.0.degrees())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapBounds {
    corners: [[f64; 2]; 2],
}

impl MapBounds {
    #[must_use]
    pub fn around(points: &[Coordinate]) -> Option<MapBounds> {
        let lons = || points.iter().map(|point| point.lon().angle().degrees());
        let lats = || points.iter().map(|point| point.lat().angle().degrees());
        points.first().map(|_| MapBounds {
            corners: [
                [
                    lons().fold(f64::INFINITY, f64::min),
                    lats().fold(f64::INFINITY, f64::min),
                ],
                [
                    lons().fold(f64::NEG_INFINITY, f64::max),
                    lats().fold(f64::NEG_INFINITY, f64::max),
                ],
            ],
        })
    }
}

impl Serialize for MapBounds {
    fn serialize<Target: Serializer>(&self, target: Target) -> Result<Target::Ok, Target::Error> {
        self.corners.serialize(target)
    }
}
