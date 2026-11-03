use guide::geo::Coordinate;
use serde::ser::{Serialize, Serializer};

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
