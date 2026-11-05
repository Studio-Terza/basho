use crate::clock::{Minutes, TripTime};
use crate::end::End;
use crate::geo::Distance;
use crate::guidance::leg_progress::LegProgress;
use crate::guidance::phase::Phase;
use crate::itinerary::Day;
use crate::leg::{Leg, LegIndex, Mode};
use std::cmp::Ordering;

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
        let progress = progress.for_day(day.number());
        match progress {
            Some(_) => Self::running(day, progress, now),
            None => Self::cold(day, now),
        }
    }

    fn cold(day: &Day, now: TripTime) -> Schedule {
        match now.trip_date().cmp(&day.date()) {
            Ordering::Equal => Self::running(day, None, now),
            Ordering::Less | Ordering::Greater => Self::preview(Self::base(day.legs(), None)),
        }
    }

    fn base(legs: &[Leg], progress: Option<LegProgress>) -> LegIndex {
        progress
            .map_or(LegIndex::default(), super::leg_progress::LegProgress::leg)
            .min(LegIndex::new(legs.len().saturating_sub(1)))
    }

    fn preview(base: LegIndex) -> Schedule {
        Schedule {
            leg: base,
            along: Distance::default(),
            phase: Phase::Preview,
            delay: None,
        }
    }

    fn running(day: &Day, progress: Option<LegProgress>, now: TripTime) -> Schedule {
        let legs = day.legs();
        let delay = progress.map_or(Minutes::ZERO, LegProgress::offset);
        let clock = now.minutes_into(now.trip_date()) - delay;
        legs.iter()
            .enumerate()
            .skip(usize::from(Self::base(legs, progress)))
            .find_map(|(index, leg)| Self::during(LegIndex::new(index), leg, clock, delay))
            .unwrap_or_else(|| Schedule {
                leg: LegIndex::new(legs.len().saturating_sub(1)),
                along: legs.last().map_or(Distance::default(), Leg::length),
                phase: Phase::Done,
                delay: Some(delay),
            })
    }

    fn during(index: LegIndex, leg: &Leg, clock: Minutes, delay: Minutes) -> Option<Schedule> {
        let start = leg.time(End::Start).since_midnight();
        match clock.cmp(&start) {
            Ordering::Less => Some(Schedule {
                leg: index,
                along: Distance::default(),
                phase: Phase::Waiting {
                    minutes: start - clock,
                },
                delay: Some(delay),
            }),
            Ordering::Equal | Ordering::Greater => Self::moving(index, leg, clock, delay),
        }
    }

    fn moving(index: LegIndex, leg: &Leg, clock: Minutes, delay: Minutes) -> Option<Schedule> {
        let start = leg.time(End::Start).since_midnight();
        let end = leg.time(End::Finish).since_midnight();
        match clock.cmp(&end) {
            Ordering::Less => Some(Schedule {
                leg: index,
                along: Self::along_at(leg, clock - start, end - start),
                phase: Phase::Moving,
                delay: Some(delay),
            }),
            Ordering::Equal | Ordering::Greater => None,
        }
    }

    fn along_at(leg: &Leg, elapsed: Minutes, span: Minutes) -> Distance {
        match leg.mode() {
            Mode::Walk => leg.along_after(elapsed),
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => Self::driven(leg, elapsed, span),
        }
    }

    fn driven(leg: &Leg, elapsed: Minutes, span: Minutes) -> Distance {
        leg.length() * (elapsed.amount() / span.max(Minutes::new(1)).amount())
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
