use serde::Deserialize;

use crate::geo::{Coordinate, Degrees, RouteLine};

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", content = "coordinates", deny_unknown_fields)]
pub enum Geometry {
    Point([Degrees; Coordinate::LON_LAT_LENGTH]),
    LineString(Vec<[Degrees; Coordinate::LON_LAT_LENGTH]>),
    MultiLineString(Vec<Vec<[Degrees; Coordinate::LON_LAT_LENGTH]>>),
}

impl Geometry {
    #[must_use]
    pub fn lines(&self) -> Vec<RouteLine> {
        match self {
            Geometry::Point(point) => vec![Self::line_through(std::slice::from_ref(point))],
            Geometry::LineString(points) => vec![Self::line_through(points)],
            Geometry::MultiLineString(lines) => lines
                .iter()
                .map(|points| Self::line_through(points))
                .collect(),
        }
    }

    fn line_through(points: &[[Degrees; Coordinate::LON_LAT_LENGTH]]) -> RouteLine {
        RouteLine::new(
            points
                .iter()
                .map(|[lon, lat]| Coordinate::new(*lat, *lon))
                .collect(),
        )
    }
}
