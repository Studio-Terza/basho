pub mod compass_point;

use std::fmt;

use strum::IntoEnumIterator;

use compass_point::CompassPoint;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bearing(f64);

impl Bearing {
    const FULL_TURN: f64 = 360.0;

    #[must_use]
    pub fn new(degrees: f64) -> Bearing {
        Bearing(degrees.rem_euclid(Self::FULL_TURN))
    }

    #[must_use]
    pub fn degrees(self) -> f64 {
        self.0
    }

    #[must_use]
    pub fn relative_to(self, facing: Bearing) -> Bearing {
        Bearing::new(self.0 - facing.0)
    }

    #[must_use]
    pub fn compass_point(self) -> CompassPoint {
        CompassPoint::iter()
            .min_by(|left, right| {
                left.center()
                    .gap_to(self)
                    .total_cmp(&right.center().gap_to(self))
            })
            .unwrap_or(CompassPoint::N)
    }

    #[must_use]
    pub fn gap_to(self, other: Bearing) -> f64 {
        let offset = (self.0 - other.0).rem_euclid(Self::FULL_TURN);
        offset.min(Self::FULL_TURN - offset)
    }
}

impl fmt::Display for Bearing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.compass_point())
    }
}
