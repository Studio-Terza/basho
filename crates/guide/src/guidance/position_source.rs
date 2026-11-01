use crate::clock::{Minutes, TripTime};
use crate::geo::Distance;
use crate::readout::Status;
use crate::times::FactTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionSource {
    Gps,
    Estimate,
    GpsLost { since: TripTime },
}

impl PositionSource {
    #[must_use]
    pub fn gps(self) -> Option<PositionSource> {
        (self == PositionSource::Gps).then_some(self)
    }

    #[must_use]
    pub fn lost_for(self, now: TripTime) -> Option<Minutes> {
        match self {
            PositionSource::GpsLost { since } => Some(now.minutes_since(since)),
            PositionSource::Gps | PositionSource::Estimate => None,
        }
    }

    #[must_use]
    pub fn last_fix_time(self) -> Option<FactTime> {
        match self {
            PositionSource::GpsLost { since } => Some(FactTime::from_gps(since)),
            PositionSource::Gps | PositionSource::Estimate => None,
        }
    }

    #[must_use]
    pub fn status(self, now: TripTime, off_route: Option<Distance>) -> Status {
        match (self, off_route) {
            (PositionSource::GpsLost { since }, _) => Status::NoGps(Some(now.minutes_since(since))),
            (PositionSource::Estimate, _) => Status::NoGps(None),
            (PositionSource::Gps, Some(distance)) => Status::OffRoute(distance),
            (PositionSource::Gps, None) => Status::OnRoute,
        }
    }
}
