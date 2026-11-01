use crate::geo::{Distance, RouteLine, Share};

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
    const GPS_ERROR: Distance = Distance::new(35.0);
    const ON_ROUTE: Distance = Distance::new(150.0);
    const CLEARLY_ELSEWHERE: Distance = Distance::new(1000.0);
    const SKETCH_SHARE: Share = Share::new(0.25);

    #[must_use]
    pub fn beside(self, line: &RouteLine) -> Distance {
        if line.is_sketch() {
            self.distance()
                .max(line.longest_segment().unwrap_or_default() * Self::SKETCH_SHARE)
        } else {
            self.distance()
        }
    }

    #[must_use]
    pub fn distance(self) -> Distance {
        match self {
            Nearness::SameRoad => Nearness::SAME_ROAD,
            Nearness::GpsError => Nearness::GPS_ERROR,
            Nearness::OffRoute | Nearness::AtStation => Nearness::GPS_ERROR + Nearness::GPS_ERROR,
            Nearness::OnRoute => Nearness::ON_ROUTE,
            Nearness::ClearlyElsewhere => Nearness::CLEARLY_ELSEWHERE,
        }
    }
}
