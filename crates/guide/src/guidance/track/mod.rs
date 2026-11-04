mod trust;
use crate::clock::{Minutes, TripTime};
use crate::geo::Coordinate;
use crate::guidance::leg_progress::LegProgress;
use crate::guidance::snap::Snap;
use crate::itinerary::Day;

pub use trust::Trust;

#[derive(Clone, Copy, Debug)]
pub struct Track {
    snap: Snap,
    kept: Option<LegProgress>,
    trust: Trust,
    here: Coordinate,
}

impl Track {
    #[must_use]
    pub fn new(snap: Snap, kept: Option<LegProgress>, trust: Trust, here: Coordinate) -> Track {
        Track {
            snap,
            kept,
            trust,
            here,
        }
    }

    #[must_use]
    pub fn snap(self) -> Snap {
        self.snap
    }

    #[must_use]
    pub fn offset(self, day: &Day, now: TripTime) -> Minutes {
        self.kept
            .filter(|kept| kept.leg() == self.snap.index())
            .map_or_else(|| self.snap.delay(day, now), LegProgress::offset)
    }

    #[must_use]
    pub fn remembered(self, day: &Day, now: TripTime) -> Option<LegProgress> {
        let along = self.snap.fix().along();
        let held = self
            .kept
            .filter(|kept| kept.leg() == self.snap.index())
            .filter(|_| self.trust == Trust::Followed)
            .map_or(along, |kept| *kept.reached().end());
        Some(self.trust)
            .filter(|trust| *trust != Trust::Unconfirmed)
            .map_or(self.kept, |_| {
                Some(
                    LegProgress::confirmed(day.number(), self.snap.index(), self.offset(day, now))
                        .placed(along, now)
                        .reaching(held)
                        .seen_at(self.here),
                )
            })
    }
}
