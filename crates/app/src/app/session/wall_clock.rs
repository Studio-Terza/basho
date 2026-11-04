use std::time::SystemTime;

use guide::clock::{TripTime, TripZone};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallClock {
    System(TripZone),
    Stopped(TripTime),
}

impl WallClock {
    #[must_use]
    pub fn time(self) -> TripTime {
        match self {
            WallClock::System(zone) => TripTime::at_instant(SystemTime::now(), zone),
            WallClock::Stopped(at) => at,
        }
    }

    #[must_use]
    pub fn moved_on(self, shown: TripTime) -> Option<TripTime> {
        let real = self.time();
        ((real.time(), real.trip_date()) != (shown.time(), shown.trip_date())).then_some(real)
    }
}
