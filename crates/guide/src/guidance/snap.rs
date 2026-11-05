use crate::clock::Pause;
use crate::clock::{Minutes, TripTime};
use crate::end::End;
use crate::geo::{Coordinate, Distance, RouteFix};
use crate::guidance::nearness::Nearness;
use crate::itinerary::Day;
use crate::leg::{Leg, LegIndex};
use newtype::Counter;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct Snap {
    index: LegIndex,
    fix: RouteFix,
}

impl Snap {
    #[must_use]
    pub fn new(index: LegIndex, fix: RouteFix) -> Snap {
        Snap { index, fix }
    }

    #[must_use]
    pub fn at_start(index: LegIndex, leg: &Leg, here: Coordinate) -> Snap {
        let start = leg.point(End::Start);
        Snap {
            index,
            fix: RouteFix::on_segment(start, start, Distance::default(), here),
        }
    }

    #[must_use]
    pub fn index(self) -> LegIndex {
        self.index
    }

    #[must_use]
    pub fn fix(self) -> RouteFix {
        self.fix
    }

    #[must_use]
    pub fn delay(self, day: &Day, now: TripTime) -> Minutes {
        let clock = now.minutes_into(day.date());
        let arrives = self.arrives(day);
        let overdue = self
            .leaves(day)
            .filter(|leaves| clock > *leaves)
            .map(|leaves| clock - leaves);
        match clock.cmp(&arrives) {
            Ordering::Less => Self::before(clock, arrives),
            Ordering::Equal | Ordering::Greater => overdue.unwrap_or(Minutes::ZERO),
        }
    }

    fn before(clock: Minutes, arrives: Minutes) -> Minutes {
        clock - arrives
    }

    fn arrives(self, day: &Day) -> Minutes {
        let slack = Nearness::GpsError.distance();
        let along = self.fix.along();
        day.leg(self.index).map_or(Minutes::ZERO, |leg| {
            let walked = leg.time(End::Start).since_midnight() + leg.planned_after(along)
                - Self::dwell_between(leg, along - slack, along);
            let before = self
                .index
                .previous()
                .and_then(|previous| {
                    let end = day.leg(previous)?.point(End::Finish);
                    let length = day.leg(previous)?.length();
                    Some(
                        Snap::new(previous, RouteFix::on_segment(end, end, length, end))
                            .arrives(day),
                    )
                })
                .unwrap_or(Minutes::ZERO);
            Self::pinned(along.cmp(&slack), walked, before)
        })
    }

    fn pinned(from_start: Ordering, walked: Minutes, before: Minutes) -> Minutes {
        match from_start {
            Ordering::Greater => walked,
            Ordering::Less | Ordering::Equal => walked.min(before),
        }
    }

    fn leaves(self, day: &Day) -> Option<Minutes> {
        let slack = Nearness::GpsError.distance();
        let along = self.fix.along();
        let leg = day.leg(self.index)?;
        let walked = leg.time(End::Start).since_midnight()
            + leg.planned_after(along)
            + Self::dwell_between(leg, along, along + slack);
        self.parted(day, leg, walked)
    }

    fn parted(self, day: &Day, leg: &Leg, walked: Minutes) -> Option<Minutes> {
        match self
            .fix
            .along()
            .cmp(&(leg.length() - Nearness::GpsError.distance()))
        {
            Ordering::Less => Some(walked),
            Ordering::Equal | Ordering::Greater => {
                Self::onward(day.leg(self.index.incremented()), leg, walked)
            }
        }
    }

    fn onward(next: Option<&Leg>, leg: &Leg, walked: Minutes) -> Option<Minutes> {
        let visit = leg.time(End::Finish).since_midnight() + leg.paused(Pause::AtEnd);
        next.map(|next| {
            walked
                .max(visit)
                .max(next.time(End::Start).since_midnight())
        })
    }

    #[expect(
        clippy::redundant_closure_for_method_calls,
        reason = "the stop type is private to the leg module"
    )]
    fn dwell_between(leg: &Leg, from: Distance, to: Distance) -> Minutes {
        leg.along()
            .iter()
            .filter(|stop| stop.distance_along() >= from)
            .filter(|stop| stop.distance_along() < to)
            .map(|stop| stop.dwell_minutes())
            .sum()
    }
}
