use crate::clock::TripTime;
use std::ops::Bound;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrivalMark {
    Arrives,
    AfterSunset,
    AfterCheckIn,
}

impl ArrivalMark {
    #[must_use]
    pub fn unarrived(self, planned: TripTime) -> Bound<TripTime> {
        match self {
            ArrivalMark::AfterSunset => Bound::Excluded(planned),
            ArrivalMark::Arrives | ArrivalMark::AfterCheckIn => Bound::Unbounded,
        }
    }
}
