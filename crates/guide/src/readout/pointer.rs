use crate::geo::{Bearing, Coordinate, Distance};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pointer {
    distance: Distance,
    bearing: Bearing,
}

impl Pointer {
    #[must_use]
    pub fn between(position: Coordinate, target: Coordinate) -> Pointer {
        Pointer {
            distance: position.distance_to(target),
            bearing: position.bearing_to(target),
        }
    }

    #[must_use]
    pub fn distance(self) -> Distance {
        self.distance
    }

    #[must_use]
    pub fn bearing(self) -> Bearing {
        self.bearing
    }
}
