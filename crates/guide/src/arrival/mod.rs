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
            ArrivalMark::Arrives => self.at,
            ArrivalMark::AfterSunset => self.sunset.filter(|limit| self.missed(mark, *limit)),
            ArrivalMark::AfterCheckIn => self.check_in.filter(|limit| self.missed(mark, *limit)),
        }
        .map(TripTime::time)
    }

    #[must_use]
    pub fn arrives(self, now: TripTime) -> Option<LiveTime> {
        self.at
            .map(|at| LiveTime::new(at.time(), Basis::PlanLeft(at.minutes_since(now))))
    }

    fn missed(self, mark: ArrivalMark, limit: TripTime) -> bool {
        self.planned <= limit
            && match mark {
                ArrivalMark::Arrives | ArrivalMark::AfterSunset => {
                    self.at.is_some_and(|at| at > limit)
                }
                ArrivalMark::AfterCheckIn => self.at.is_none_or(|at| at > limit),
            }
    }
}
