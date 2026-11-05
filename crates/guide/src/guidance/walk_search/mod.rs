mod pick;
mod reach;
mod recovering;
mod stride;
use crate::clock::TripTime;
use crate::end::End;
use crate::geo::{Coordinate, Distance, RouteFix};
use crate::guidance::leg_progress::LegProgress;
use crate::guidance::nearness::Nearness;
use crate::guidance::snap::Snap;
use crate::guidance::track::{Track, Trust};
use crate::itinerary::Day;
use crate::leg::{Leg, LegIndex, Mode};
use newtype::Counter;
use pick::Pick;
use reach::Reach;
use recovering::Recovering;
use std::cmp::Ordering;
use stride::Stride;

#[derive(Clone, Copy, Debug)]
pub struct WalkSearch<'day> {
    day: &'day Day,
    here: Coordinate,
    now: TripTime,
}

impl<'day> WalkSearch<'day> {
    #[must_use]
    pub fn new(day: &'day Day, here: Coordinate, now: TripTime) -> WalkSearch<'day> {
        WalkSearch { day, here, now }
    }

    #[must_use]
    pub fn best(self, kept: Option<LegProgress>) -> Option<Track> {
        self.searched(&self.candidates(), kept)
    }

    fn searched(self, near: &[Snap], kept: Option<LegProgress>) -> Option<Track> {
        match kept.filter(|kept| self.day.leg(kept.leg()).is_some()) {
            Some(kept) => self.continued(near, kept),
            None => self.unseen(near),
        }
    }

    fn unseen(self, near: &[Snap]) -> Option<Track> {
        Pick::unseen(self.day, self.here, self.now)
            .best(near)
            .map(|snap| self.tracked(snap, None, Trust::Followed))
    }

    #[must_use]
    pub fn candidates(self) -> Vec<Snap> {
        self.day
            .legs()
            .iter()
            .enumerate()
            .filter(|(_, leg)| leg.mode() == Mode::Walk)
            .flat_map(|(position, leg)| {
                let reach = Nearness::OnRoute.beside(leg.line());
                let fixes = leg.line().fixes(self.here);
                let line = fixes
                    .iter()
                    .min_by_key(|fix| fix.off_route())
                    .map_or(reach, |fix| Distance::from(fix.off_route()));
                let stops = leg
                    .along()
                    .iter()
                    .map(|stop| {
                        RouteFix::on_segment(
                            Coordinate::from(stop),
                            Coordinate::from(stop),
                            stop.distance_along(),
                            self.here,
                        )
                    })
                    .filter(move |stop| stop.off_route() < line);
                fixes
                    .into_iter()
                    .chain(stops)
                    .filter(move |fix| fix.off_route() <= reach)
                    .map(move |fix| Snap::new(LegIndex::new(position), fix))
            })
            .collect()
    }

    fn continued(self, near: &[Snap], kept: LegProgress) -> Option<Track> {
        let best = near
            .iter()
            .map(|snap| Distance::from(snap.fix().off_route()))
            .min_by(|left, right| left.meters().total_cmp(&right.meters()))?
            + Nearness::GpsError.distance();
        let reachable: Vec<Snap> = near
            .iter()
            .copied()
            .filter(|snap| snap.fix().off_route() <= best)
            .filter(|snap| {
                Stride::between(self.day, kept, *snap, self.now)
                    .is_some_and(|stride| stride.reach() == Reach::Within)
            })
            .collect();
        Pick::new(self.day, self.here, self.now, kept)
            .best(&reachable)
            .map(|snap| self.tracked(snap, Some(kept), Trust::Followed))
            .or_else(|| self.recovered(near, kept, best))
    }

    fn recovered(self, near: &[Snap], kept: LegProgress, best: Distance) -> Option<Track> {
        let recovering = Recovering::new(self, near, kept, best);
        match self.whereabouts(kept) {
            Whereabouts::Elsewhere => recovering.turned_within(near),
            Whereabouts::Beside => recovering.aboard(),
        }
    }

    fn whereabouts(self, kept: LegProgress) -> Whereabouts {
        match self.day.leg(kept.leg()) {
            None => Whereabouts::Elsewhere,
            Some(leg) => self.along_line(leg),
        }
    }

    fn along_line(self, leg: &Leg) -> Whereabouts {
        match leg.line().fix(self.here) {
            None => Whereabouts::Elsewhere,
            Some(fix) => Whereabouts::from(
                Distance::from(fix.off_route()).cmp(&Nearness::ClearlyElsewhere.beside(leg.line())),
            ),
        }
    }

    fn tracked(self, snap: Snap, kept: Option<LegProgress>, trust: Trust) -> Track {
        let leg = self.day.leg(snap.index());
        let ended = leg
            .filter(|leg| snap.fix().along() >= leg.length() - Nearness::GpsError.distance())
            .filter(|leg| {
                self.here.distance_to(leg.point(End::Finish)) <= Nearness::OffRoute.distance()
            })
            .and_then(|_| self.day.leg(snap.index().incremented()))
            .filter(|next| next.mode() == Mode::Walk)
            .map(|next| Snap::at_start(snap.index().incremented(), next, self.here));
        let shown = ended.unwrap_or(snap);
        Track::new(shown, kept, self.confirmed(snap, trust), self.here)
    }

    fn confirmed(self, snap: Snap, trust: Trust) -> Trust {
        match self.day.leg(snap.index()).map(|leg| {
            Distance::from(snap.fix().off_route()).cmp(&Nearness::OffRoute.beside(leg.line()))
        }) {
            Some(Ordering::Greater) => Trust::Unconfirmed,
            Some(Ordering::Less | Ordering::Equal) | None => trust,
        }
    }
}

enum Whereabouts {
    Elsewhere,
    Beside,
}

impl From<Ordering> for Whereabouts {
    fn from(against_reach: Ordering) -> Whereabouts {
        match against_reach {
            Ordering::Greater => Whereabouts::Elsewhere,
            Ordering::Less | Ordering::Equal => Whereabouts::Beside,
        }
    }
}
