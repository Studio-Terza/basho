use super::{Placing, Stage};
use crate::end::End;
use crate::geo::Distance;
use crate::guidance::LegProgress;
use crate::guidance::{Guidance, Phase, PositionSource};
use crate::leg::Leg;

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
        let waiting = self.day.leg(floor.on_leg()).map(|leg| {
            let start = leg.time(End::Start).since_midnight();
            Some(start)
                .filter(|start| clock < *start)
                .map_or(Phase::Overdue, |start| Phase::Waiting {
                    minutes: start - clock,
                })
        });
        let held = Guidance {
            leg: floor.on_leg(),
            along: Some(self.leg)
                .filter(|leg| *leg == floor.on_leg())
                .map_or(Distance::default(), |_| self.along),
            source: PositionSource::Estimate,
            off_route: None,
            ..self
        };
        match floor.stage() {
            Stage::Finished => held.arrived(),
            Stage::Boarding => held.in_phase(waiting.unwrap_or(Phase::Moving)),
            Stage::Under => held.in_phase(Phase::Moving),
        }
    }

    fn in_phase(self, phase: Phase) -> Guidance<'day> {
        Guidance { phase, ..self }
    }
}
