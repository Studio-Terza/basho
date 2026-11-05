use crate::clock::{Minutes, TripTime};
use crate::geo::Distance;
use crate::guidance::arrow::{Arrow, Freshness};
use crate::guidance::time_check::TimeCheck;
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
    pub fn age(self, now: TripTime) -> Freshness {
        match self {
            PositionSource::GpsLost { since } => {
                Freshness::from(now.minutes_since(since).cmp(&Arrow::STALE))
            }
            PositionSource::Gps | PositionSource::Estimate => Freshness::Fresh,
        }
    }

    #[must_use]
    pub fn checked(self, off_route: Option<Distance>, check: TimeCheck) -> TimeCheck {
        match self {
            PositionSource::Gps => check,
            PositionSource::GpsLost { .. } | PositionSource::Estimate => {
                check.relocated(off_route, self.last_fix_time())
            }
        }
    }

    #[must_use]
    pub fn status(self, now: TripTime, off_route: Option<Distance>) -> Status {
        match self {
            PositionSource::GpsLost { since } => Status::NoGps(Some(now.minutes_since(since))),
            PositionSource::Estimate => Status::NoGps(None),
            PositionSource::Gps => off_route.map_or(Status::OnRoute, Status::OffRoute),
        }
    }
}
