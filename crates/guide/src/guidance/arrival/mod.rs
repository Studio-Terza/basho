mod arrival_mark;
use crate::clock::{ClockTime, TripTime};
use crate::times::{Basis, LiveTime};
pub use arrival_mark::ArrivalMark;

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
            ArrivalMark::AfterSunset => self
                .limited(self.sunset)
                .filter(|limit| Some(*limit) < self.at)
                .map(TripTime::time),
            ArrivalMark::AfterCheckIn => self
                .limited(self.check_in)
                .filter(|limit| Some(*limit) < self.at)
                .or(self.limited(self.check_in).filter(|_| self.at.is_none()))
                .map(TripTime::time),
        }
    }

    fn limited(self, limit: Option<TripTime>) -> Option<TripTime> {
        limit.filter(|limit| self.planned <= *limit)
    }

    #[must_use]
    pub fn arrives(self, now: TripTime) -> Option<LiveTime> {
        self.at
            .map(|at| LiveTime::new(at.time(), Basis::PlanLeft(at.minutes_since(now))))
    }
}
