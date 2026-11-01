use crate::clock::TripTime;
use crate::geo::{Coordinate, Distance};
use crate::guidance::{Guidance, LegProgress};
use crate::itinerary::Day;
use crate::readout::Trail;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sighting {
    Fresh(Coordinate),
    Stale { at: Coordinate, since: TripTime },
}

impl Sighting {
    #[must_use]
    pub fn fresh(self) -> Option<Coordinate> {
        match self {
            Sighting::Fresh(at) => Some(at),
            Sighting::Stale { .. } => None,
        }
    }

    #[must_use]
    pub fn kept_over(self, latest: Sighting, step: Distance) -> Sighting {
        self.fresh()
            .zip(latest.fresh())
            .filter(|(before, after)| before.distance_to(*after) < step)
            .map_or(latest, |_| self)
    }

    #[must_use]
    pub fn place(self) -> Coordinate {
        match self {
            Sighting::Fresh(at) | Sighting::Stale { at, .. } => at,
        }
    }

    #[must_use]
    pub fn last_fix(self) -> Option<TripTime> {
        match self {
            Sighting::Fresh(_) => None,
            Sighting::Stale { since, .. } => Some(since),
        }
    }

    #[must_use]
    pub fn aged(self, now: TripTime) -> Sighting {
        match self {
            Sighting::Fresh(at) => Sighting::Stale { at, since: now },
            Sighting::Stale { .. } => self,
        }
    }

    #[must_use]
    pub fn recorded_on(self, trail: Trail) -> Trail {
        match self {
            Sighting::Fresh(at) => trail.recorded(at),
            Sighting::Stale { .. } => trail,
        }
    }

    #[must_use]
    pub fn guidance(self, day: &Day, progress: LegProgress, now: TripTime) -> Guidance<'_> {
        match self {
            Sighting::Fresh(at) => Guidance::new(day, progress, now, Some(at)),
            Sighting::Stale { since, .. } if since.trip_date() != now.trip_date() => {
                Guidance::new(day, progress, now, None)
            }
            Sighting::Stale { at, since } => Guidance::new(day, progress, since, Some(at))
                .after_gps_loss(Guidance::new(day, progress, now, None)),
        }
    }
}
