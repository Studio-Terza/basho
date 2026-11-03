use guide::geo::Bearing;
use serde::ser::{Serialize, Serializer};

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
