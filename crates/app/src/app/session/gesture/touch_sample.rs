use std::time::Instant;

use guide::plane::PlanePoint;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TouchSample {
    at: PlanePoint,
    time: Instant,
}

impl TouchSample {
    #[must_use]
    pub fn new(at: PlanePoint, time: Instant) -> TouchSample {
        TouchSample { at, time }
    }

    #[must_use]
    pub fn point(self) -> PlanePoint {
        self.at
    }

    #[must_use]
    pub fn time(self) -> Instant {
        self.time
    }
}
