mod arrival_mark;
use crate::clock::{ClockTime, TripTime};
use crate::times::{Basis, LiveTime};
pub use arrival_mark::ArrivalMark;
use std::ops::{Bound, RangeBounds};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arrival {
    planned: TripTime,
    at: Option<TripTime>,
    sunset: Option<TripTime>,
    check_in: Option<TripTime>,
}

impl Arrival {
    #[must_use]
    pub fn new(
        planned: TripTime,
        at: Option<TripTime>,
        sunset: Option<TripTime>,
        check_in: Option<TripTime>,
    ) -> Arrival {
        Arrival {
            planned,
            at,
            sunset,
            check_in,
        }
    }

    #[must_use]
    pub fn time(self, mark: ArrivalMark) -> Option<ClockTime> {
        match mark {
            ArrivalMark::Arrives => self.at.map(TripTime::time),
            ArrivalMark::AfterSunset => self.passed(self.sunset, mark),
            ArrivalMark::AfterCheckIn => self.passed(self.check_in, mark),
        }
    }

    fn passed(self, limit: Option<TripTime>, mark: ArrivalMark) -> Option<ClockTime> {
        limit
            .filter(|limit| {
                (
                    Bound::Included(self.planned),
                    self.at
                        .map_or(mark.unarrived(self.planned), Bound::Excluded),
                )
                    .contains(limit)
            })
            .map(TripTime::time)
    }

    #[must_use]
    pub fn arrives(self, now: TripTime) -> Option<LiveTime> {
        self.at
            .map(|at| LiveTime::new(at.time(), Basis::PlanLeft(at.minutes_since(now))))
    }
}
