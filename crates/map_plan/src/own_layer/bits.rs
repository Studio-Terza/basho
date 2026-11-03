use guide::geo::Coordinate;
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
