use std::fmt;

use map_class::GridAxis;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct LonLat(f64, f64);

impl LonLat {
    #[must_use]
    pub const fn new(lon: f64, lat: f64) -> LonLat {
        LonLat(lon, lat)
    }

    #[must_use]
    pub const fn degrees_along(self, axis: GridAxis) -> f64 {
        match axis {
            GridAxis::Across => self.0,
            GridAxis::Down => self.1,
        }
    }

    #[must_use]
    pub const fn square_to(self, opposite: LonLat) -> [LonLat; 5] {
        [
            LonLat(self.0, opposite.1),
            opposite,
            LonLat(opposite.0, self.1),
            self,
            LonLat(self.0, opposite.1),
        ]
    }
}

impl fmt::Display for LonLat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{},{}]", self.0, self.1)
    }
}
