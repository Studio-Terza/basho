use super::{Placing, Stage};
use crate::clock::Minutes;
use crate::end::End;
use crate::geo::Distance;
use crate::guidance::LegProgress;
use crate::guidance::{Guidance, Phase, PositionSource};
use crate::leg::Leg;
use std::cmp::Ordering;

impl<'day> Guidance<'day> {
    #[must_use]
    pub fn placing(&self) -> Placing {
        Placing::new(self.leg, self.phase.staged(self.leg().map(Leg::mode)))
    }

    #[must_use]
    pub fn held(self, progress: LegProgress) -> Guidance<'day> {
        progress
            .for_day(self.day.number())
            .and_then(|kept| kept.tracking().holding(self.placing(), self.source))
            .map_or(self, |floor| self.floored(floor))
    }

    fn floored(self, floor: Placing) -> Guidance<'day> {
        let clock = self.now.minutes_into(self.day.date()) - self.offset;
        let waiting = self
            .day
            .leg(floor.on_leg())
            .map(|leg| Self::waiting_for(clock, leg.time(End::Start).since_midnight()));
        Guidance {
            leg: floor.on_leg(),
            along: Self::along_on(self.leg.cmp(&floor.on_leg()), self.along),
            source: PositionSource::Estimate,
            off_route: None,
            ..self
        }
        .staged(floor.stage(), waiting)
    }

    fn waiting_for(clock: Minutes, start: Minutes) -> Phase {
        match clock.cmp(&start) {
            Ordering::Less => Phase::Waiting {
                minutes: start - clock,
            },
            Ordering::Equal | Ordering::Greater => Phase::Overdue,
        }
    }

    fn along_on(same_leg: Ordering, along: Distance) -> Distance {
        match same_leg {
            Ordering::Equal => along,
            Ordering::Less | Ordering::Greater => Distance::default(),
        }
    }

    fn staged(self, stage: Stage, waiting: Option<Phase>) -> Guidance<'day> {
        match stage {
            Stage::Finished => self.arrived(),
            Stage::Boarding => self.in_phase(waiting.unwrap_or(Phase::Moving)),
            Stage::Under => self.in_phase(Phase::Moving),
        }
    }

    fn in_phase(self, phase: Phase) -> Guidance<'day> {
        Guidance { phase, ..self }
    }
}
