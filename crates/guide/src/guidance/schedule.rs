use crate::clock::{Minutes, TripTime};
use crate::end::End;
use crate::geo::Distance;
use crate::guidance::leg_progress::LegProgress;
use crate::guidance::phase::Phase;
use crate::itinerary::Day;
use crate::leg::{Leg, LegIndex, Mode};

#[derive(derive_more::AsRef, Clone, Copy, Debug)]
pub struct Schedule {
    #[as_ref]
    leg: LegIndex,
    #[as_ref]
    along: Distance,
    phase: Phase,
    delay: Option<Minutes>,
}

impl Schedule {
    #[must_use]
    pub fn estimate(day: &Day, progress: LegProgress, now: TripTime) -> Schedule {
        let legs = day.legs();
        let progress = progress.for_day(day.number());
        let base = progress
            .map_or(LegIndex::default(), super::leg_progress::LegProgress::leg)
            .min(LegIndex::new(legs.len().saturating_sub(1)));
        if now.trip_date() != day.date() && progress.is_none() {
            Schedule {
                leg: base,
                along: Distance::default(),
                phase: Phase::Preview,
                delay: None,
            }
        } else {
            let delay = progress.map_or(Minutes::ZERO, LegProgress::offset);
            let clock = now.minutes_into(now.trip_date()) - delay;
            legs.iter()
                .enumerate()
                .skip(usize::from(base))
                .find_map(|(index, leg)| Self::during(LegIndex::new(index), leg, clock, delay))
                .unwrap_or_else(|| Schedule {
                    leg: LegIndex::new(legs.len().saturating_sub(1)),
                    along: legs.last().map_or(Distance::default(), Leg::length),
                    phase: Phase::Done,
                    delay: Some(delay),
                })
        }
    }

    fn during(index: LegIndex, leg: &Leg, clock: Minutes, delay: Minutes) -> Option<Schedule> {
        let start = leg.time(End::Start).since_midnight();
        let end = leg.time(End::Finish).since_midnight();
        if clock < start {
            Some(Schedule {
                leg: index,
                along: Distance::default(),
                phase: Phase::Waiting {
                    minutes: start - clock,
                },
                delay: Some(delay),
            })
        } else {
            (clock < end).then(|| Schedule {
                leg: index,
                along: Self::along_at(leg, clock - start, end - start),
                phase: Phase::Moving,
                delay: Some(delay),
            })
        }
    }

    fn along_at(leg: &Leg, elapsed: Minutes, span: Minutes) -> Distance {
        match leg.mode() {
            Mode::Walk => leg.along_after(elapsed),
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => {
                leg.length() * (elapsed.amount() / span.max(Minutes::new(1)).amount())
            }
        }
    }

    #[must_use]
    pub fn phase(self) -> Phase {
        self.phase
    }

    #[must_use]
    pub fn delay(self) -> Option<Minutes> {
        self.delay
    }
}
