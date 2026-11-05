use crate::geo::Distance;
use crate::guidance::leg_progress::LegProgress;
use crate::guidance::nearness::Nearness;
use crate::guidance::snap::Snap;
use crate::guidance::track::{Track, Trust};
use crate::guidance::walk_search::WalkSearch;
use crate::guidance::walk_search::pick::Pick;
use crate::guidance::walk_search::reach::Reach;
use crate::guidance::walk_search::stride::Stride;
use crate::leg::{Leg, Mode};

pub struct Recovering<'day> {
    search: WalkSearch<'day>,
    pick: Pick<'day>,
    kept: LegProgress,
    best: Distance,
    others: Vec<Snap>,
    current: Vec<Snap>,
    settled: Vec<Snap>,
}

impl<'day> Recovering<'day> {
    pub fn new(
        search: WalkSearch<'day>,
        near: &[Snap],
        kept: LegProgress,
        best: Distance,
    ) -> Recovering<'day> {
        let recovering = Recovering {
            search,
            pick: Pick::new(search.day, search.here, search.now, kept),
            kept,
            best,
            others: Vec::new(),
            current: Vec::new(),
            settled: Vec::new(),
        };
        Recovering {
            others: near
                .iter()
                .copied()
                .filter(|snap| snap.index() != kept.leg())
                .filter(|snap| snap.fix().off_route() <= best)
                .collect(),
            current: near
                .iter()
                .copied()
                .filter(|snap| snap.index() == kept.leg())
                .collect(),
            settled: near
                .iter()
                .copied()
                .filter(|snap| {
                    recovering
                        .reach_of(snap)
                        .is_some_and(|reach| reach != Reach::Beyond)
                })
                .collect(),
            ..recovering
        }
    }

    pub fn turned_within(&self, pool: &[Snap]) -> Option<Track> {
        self.pick
            .best(pool)
            .map(|snap| self.search.tracked(snap, Some(self.kept), Trust::Turned))
    }

    pub fn aboard(&self) -> Option<Track> {
        match self.search.day.leg(self.kept.leg()).map(Leg::mode) {
            Some(Mode::Train | Mode::Bus | Mode::Boat | Mode::Car) => {
                self.turned_within(&self.settled)
            }
            Some(Mode::Walk) | None => self.overtaken(),
        }
    }

    fn overtaken(&self) -> Option<Track> {
        match self.others.first() {
            Some(_) => self.passed_by(),
            None => self.turned_back(),
        }
    }

    fn passed_by(&self) -> Option<Track> {
        match self
            .current
            .iter()
            .filter(|snap| snap.fix().off_route() <= self.best)
            .find(|snap| self.reach_of(snap) == Some(Reach::Behind))
        {
            Some(_) => self.turned_within(&self.others),
            None => self.turned_back(),
        }
    }

    fn turned_back(&self) -> Option<Track> {
        match self
            .others
            .iter()
            .find(|snap| self.reach_of(snap) == Some(Reach::Behind))
        {
            Some(_) => self.cleared(),
            None => self.following(),
        }
    }

    fn cleared(&self) -> Option<Track> {
        match self
            .current
            .iter()
            .find(|snap| snap.fix().off_route() <= Nearness::OffRoute.distance())
        {
            Some(_) => self.following(),
            None => self.turned_within(&self.behind()),
        }
    }

    fn behind(&self) -> Vec<Snap> {
        self.others
            .iter()
            .copied()
            .filter(|snap| self.reach_of(snap) == Some(Reach::Behind))
            .collect()
    }

    fn following(&self) -> Option<Track> {
        self.pick.best(&self.current).map(|snap| {
            self.search.tracked(
                snap,
                Some(self.kept),
                Stride::between(self.search.day, self.kept, snap, self.search.now)
                    .map_or(Trust::Unconfirmed, Stride::returning),
            )
        })
    }

    fn reach_of(&self, snap: &Snap) -> Option<Reach> {
        Stride::between(self.search.day, self.kept, *snap, self.search.now).map(Stride::reach)
    }
}
