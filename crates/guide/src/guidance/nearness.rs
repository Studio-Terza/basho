use crate::geo::{Distance, RouteLine, Share};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nearness {
    SameRoad,
    GpsError,
    OffRoute,
    OnRoute,
    AtStation,
    ClearlyElsewhere,
}

impl Nearness {
    const SAME_ROAD: Distance = Distance::new(10.0);
    const GPS_ERROR_METERS: f64 = 35.0;
    const GPS_ERROR: Distance = Distance::new(Self::GPS_ERROR_METERS);
    const OFF_ROUTE: Distance = Distance::new(2.0 * Self::GPS_ERROR_METERS);
    const ON_ROUTE: Distance = Distance::new(150.0);
    const CLEARLY_ELSEWHERE: Distance = Distance::new(1000.0);
    const SKETCH_SHARE: Share = Share::new(0.25);

    #[must_use]
    pub fn beside(self, line: &RouteLine) -> Distance {
        line.longest_segment()
            .map_or(self.distance(), |longest| self.stretched(longest))
    }

    fn stretched(self, longest: Distance) -> Distance {
        match longest.cmp(&RouteLine::SKETCH_SEGMENT) {
            Ordering::Greater => self.distance().max(longest * Self::SKETCH_SHARE),
            Ordering::Less | Ordering::Equal => self.distance(),
        }
    }

    #[must_use]
    pub fn distance(self) -> Distance {
        match self {
            Nearness::SameRoad => Nearness::SAME_ROAD,
            Nearness::GpsError => Nearness::GPS_ERROR,
            Nearness::OffRoute | Nearness::AtStation => Nearness::OFF_ROUTE,
            Nearness::OnRoute => Nearness::ON_ROUTE,
            Nearness::ClearlyElsewhere => Nearness::CLEARLY_ELSEWHERE,
        }
    }
}
