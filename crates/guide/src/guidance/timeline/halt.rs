use crate::clock::{Minutes, TripTime};
use crate::geo::{Coordinate, Distance, Share};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Halt {
    at: Coordinate,
    since: TripTime,
}

impl Halt {
    pub const STAY: Distance = Distance::new(30.0);
    pub const JITTER: Distance = Distance::new(30.0);
    pub const STILL: Minutes = Minutes::new(3);
    const SETTLE: Share = Share::new(0.2);

    #[must_use]
    pub fn tracked(previous: Option<Halt>, at: Coordinate, now: TripTime) -> Halt {
        previous
            .filter(|halt| halt.at.distance_to(at) <= Self::STAY + Self::JITTER)
            .map_or(Halt { at, since: now }, |halt| Halt {
                at: halt.at.toward(at, Self::SETTLE),
                ..halt
            })
    }

    #[must_use]
    pub fn location(self) -> Coordinate {
        self.at
    }

    #[must_use]
    pub fn still_for(self, now: TripTime) -> Option<Minutes> {
        Some(now.minutes_since(self.since)).filter(|still| *still > Self::STILL)
    }
}
