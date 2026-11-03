use std::ops::Index;

use crate::plane::axis::Axis;
use crate::plane::pixels::Pixels;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanePoint {
    across: Pixels,
    down: Pixels,
}

impl PlanePoint {
    #[must_use]
    #[inline]
    pub const fn new(across: Pixels, down: Pixels) -> PlanePoint {
        PlanePoint { across, down }
    }

    #[must_use]
    pub fn moved(self, right: Pixels, down: Pixels) -> PlanePoint {
        PlanePoint::new(self.across + right, self.down + down)
    }

    #[must_use]
    pub fn distance_to(self, other: PlanePoint) -> Pixels {
        (self.across - other.across).hypot(self.down - other.down)
    }
}

impl Index<Axis> for PlanePoint {
    type Output = Pixels;

    #[inline]
    fn index(&self, axis: Axis) -> &Pixels {
        match axis {
            Axis::Across => &self.across,
            Axis::Down => &self.down,
        }
    }
}
