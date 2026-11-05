use crate::Stance;
use crate::geo::Coordinate;
use crate::guidance::Guidance;
use crate::guidance::Reach;
use crate::guidance::walk_search::WalkSearch;

impl<'day> Guidance<'day> {
    pub(super) fn reached(self, reach: Reach, position: Option<Coordinate>) -> Guidance<'day> {
        match reach {
            Reach::Arrived => self.arrived(),
            Reach::Underway => self.placed(position),
        }
    }

    fn placed(self, position: Option<Coordinate>) -> Guidance<'day> {
        match position {
            Some(here) => self.positioned(here),
            None => self,
        }
    }

    fn positioned(self, here: Coordinate) -> Guidance<'day> {
        match self.phase.stance() {
            Stance::Preview => WalkSearch::new(self.day, here, self.now)
                .best(None)
                .map_or(self, |track| self.moving(track)),
            Stance::Waiting | Stance::Moving | Stance::Missed | Stance::Done => {
                self.located(here, self.kept.for_day(self.day.number()))
            }
        }
    }
}
