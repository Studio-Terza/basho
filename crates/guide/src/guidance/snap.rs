use crate::clock::Pause;
use crate::clock::{Minutes, TripTime};
use crate::end::End;
use crate::geo::{Coordinate, Distance, RouteFix};
use crate::guidance::nearness::Nearness;
use crate::itinerary::Day;
use crate::leg::{Leg, LegIndex};
use newtype::Counter;

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
        if clock < arrives {
            clock - arrives
        } else {
            overdue.unwrap_or(Minutes::ZERO)
        }
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
            if along <= slack {
                walked.min(before)
            } else {
                walked
            }
        })
    }

    fn leaves(self, day: &Day) -> Option<Minutes> {
        let slack = Nearness::GpsError.distance();
        let along = self.fix.along();
        let leg = day.leg(self.index)?;
        let walked = leg.time(End::Start).since_midnight()
            + leg.planned_after(along)
            + Self::dwell_between(leg, along, along + slack);
        let visit = leg.time(End::Finish).since_midnight() + leg.paused(Pause::AtEnd);
        if along >= leg.length() - slack {
            day.leg(self.index.incremented()).map(|next| {
                walked
                    .max(visit)
                    .max(next.time(End::Start).since_midnight())
            })
        } else {
            Some(walked)
        }
    }

    #[expect(
        clippy::redundant_closure_for_method_calls,
        reason = "the stop type is private to the leg module"
    )]
    fn dwell_between(leg: &Leg, from: Distance, to: Distance) -> Minutes {
        leg.along()
            .iter()
            .filter(|stop| stop.distance_along() >= from && stop.distance_along() < to)
            .map(|stop| stop.dwell_minutes())
            .sum()
    }
}
