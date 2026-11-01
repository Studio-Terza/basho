mod pick;
mod reach;
mod stride;
use crate::clock::TripTime;
use crate::end::End;
use crate::geo::{Coordinate, Distance, RouteFix};
use crate::guidance::leg_progress::LegProgress;
use crate::guidance::nearness::Nearness;
use crate::guidance::snap::Snap;
use crate::guidance::track::{Track, Trust};
use crate::itinerary::Day;
use crate::leg::{LegIndex, Mode};
use newtype::Counter;
use pick::Pick;
use reach::Reach;
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
        let near = self.candidates();
        match kept.filter(|kept| self.day.leg(kept.leg()).is_some()) {
            Some(kept) => self.continued(&near, kept),
            None => Pick::unseen(self.day, self.here, self.now)
                .best(&near)
                .map(|snap| self.tracked(snap, None, Trust::Followed)),
        }
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
        let pick = Pick::new(self.day, self.here, self.now, kept);
        let behind = |snap: &Snap| {
            Stride::between(self.day, kept, *snap, self.now)
                .is_some_and(|stride| stride.reach() == Reach::Behind)
        };
        let others: Vec<Snap> = near
            .iter()
            .copied()
            .filter(|snap| snap.index() != kept.leg() && snap.fix().off_route() <= best)
            .collect();
        let current: Vec<Snap> = near
            .iter()
            .copied()
            .filter(|snap| snap.index() == kept.leg())
            .collect();
        let settled: Vec<Snap> = near
            .iter()
            .copied()
            .filter(|snap| {
                Stride::between(self.day, kept, *snap, self.now)
                    .is_some_and(|stride| stride.reach() != Reach::Beyond)
            })
            .collect();
        if self.elsewhere(kept) {
            pick.best(near)
                .map(|snap| self.tracked(snap, Some(kept), Trust::Turned))
        } else if self
            .day
            .leg(kept.leg())
            .is_some_and(|leg| leg.mode() != Mode::Walk)
        {
            pick.best(&settled)
                .map(|snap| self.tracked(snap, Some(kept), Trust::Turned))
        } else if !others.is_empty()
            && current
                .iter()
                .any(|snap| snap.fix().off_route() <= best && behind(snap))
        {
            pick.best(&others)
                .map(|snap| self.tracked(snap, Some(kept), Trust::Turned))
        } else if others.iter().any(behind)
            && current
                .iter()
                .all(|snap| snap.fix().off_route() > Nearness::OffRoute.distance())
        {
            let back: Vec<Snap> = others.iter().copied().filter(behind).collect();
            pick.best(&back)
                .map(|snap| self.tracked(snap, Some(kept), Trust::Turned))
        } else {
            pick.best(&current).map(|snap| {
                self.tracked(
                    snap,
                    Some(kept),
                    Stride::between(self.day, kept, snap, self.now)
                        .map_or(Trust::Unconfirmed, Stride::returning),
                )
            })
        }
    }

    fn elsewhere(self, kept: LegProgress) -> bool {
        self.day.leg(kept.leg()).is_none_or(|leg| {
            leg.line()
                .fix(self.here)
                .is_none_or(|fix| fix.off_route() > Nearness::ClearlyElsewhere.beside(leg.line()))
        })
    }

    fn tracked(self, snap: Snap, kept: Option<LegProgress>, trust: Trust) -> Track {
        let leg = self.day.leg(snap.index());
        let ended = leg
            .filter(|leg| {
                snap.fix().along() >= leg.length() - Nearness::GpsError.distance()
                    && self.here.distance_to(leg.point(End::Finish))
                        <= Nearness::OffRoute.distance()
            })
            .and_then(|_| self.day.leg(snap.index().incremented()))
            .filter(|next| next.mode() == Mode::Walk)
            .map(|next| Snap::at_start(snap.index().incremented(), next, self.here));
        let shown = ended.unwrap_or(snap);
        let trust = if leg
            .is_some_and(|leg| snap.fix().off_route() > Nearness::OffRoute.beside(leg.line()))
        {
            Trust::Unconfirmed
        } else {
            trust
        };
        Track::new(shown, kept, trust, self.here)
    }
}
