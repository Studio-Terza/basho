use crate::clock::{Minutes, TripTime};
use crate::end::End;
use crate::geo::{Distance, Speed};
use crate::guidance::leg_progress::LegProgress;
use crate::guidance::nearness::Nearness;
use crate::guidance::snap::Snap;
use crate::guidance::track::Trust;
use crate::guidance::walk_search::reach::Reach;
use crate::itinerary::Day;
use crate::leg::{Leg, LegIndex, Mode};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stride {
    walked: Distance,
    stepped: Distance,
    budget: Option<Distance>,
}

impl Stride {
    pub const BRISK: Speed = Speed::kilometers_per_hour(6.0);

    #[must_use]
    pub fn between(day: &Day, kept: LegProgress, snap: Snap, now: TripTime) -> Option<Stride> {
        let clock = now.minutes_into(day.date());
        let crossed: Vec<&Leg> = (usize::from(kept.leg())..usize::from(snap.index()))
            .filter_map(|position| day.leg(LegIndex::new(position)))
            .filter(|leg| leg.mode() != Mode::Walk)
            .collect();
        let since = crossed
            .iter()
            .map(|ride| clock - ride.time(End::Finish).since_midnight())
            .chain(Some(kept.minutes_before(day, now)))
            .min();
        crossed
            .iter()
            .all(|ride| clock >= ride.time(End::Start).since_midnight())
            .then(|| Stride {
                walked: Self::position(day, snap.index(), snap.fix().along())
                    - Self::position(day, kept.leg(), *kept.reached().end()),
                stepped: Self::position(day, snap.index(), snap.fix().along())
                    - Self::position(day, kept.leg(), kept.along()),
                budget: since.map(|minutes| {
                    Self::BRISK.distance_in(minutes.max(Minutes::ZERO))
                        + Nearness::OffRoute.distance()
                }),
            })
    }

    #[must_use]
    pub fn stepped(self) -> Distance {
        self.stepped
    }

    #[must_use]
    pub fn reach(self) -> Reach {
        match self.budget {
            _ if self.walked < -Nearness::GpsError.distance() => Reach::Behind,
            Some(budget) if self.walked > budget => Reach::Beyond,
            Some(_) | None => Reach::Within,
        }
    }

    #[must_use]
    pub fn returning(self) -> Trust {
        match self.reach() {
            Reach::Behind => Trust::Turned,
            Reach::Within | Reach::Beyond => Trust::Unconfirmed,
        }
    }

    fn position(day: &Day, index: LegIndex, along: Distance) -> Distance {
        let before: Distance = day
            .legs()
            .iter()
            .take(usize::from(index))
            .filter(|leg| leg.mode() == Mode::Walk)
            .map(Leg::length)
            .sum();
        day.leg(index)
            .filter(|leg| leg.mode() == Mode::Walk)
            .map_or(before, |_| before + along)
    }
}
