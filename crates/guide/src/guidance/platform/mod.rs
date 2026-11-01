use crate::clock::{Minutes, TripTime};
use crate::end::End;
use crate::geo::Coordinate;
use crate::guidance::nearness::Nearness;
use crate::guidance::phase::Phase;
use crate::guidance::ride_slack::RideSlack;
use crate::guidance::snap::Snap;
use crate::itinerary::Day;
use crate::leg::{Leg, LegIndex, Mode};
use newtype::Counter;

#[derive(Clone, Copy, Debug)]
pub struct Platform<'day> {
    ride: LegIndex,
    leg: &'day Leg,
    walking: Option<Snap>,
}

impl<'day> Platform<'day> {
    #[must_use]
    pub fn arriving(day: &'day Day, walking: Option<Snap>) -> Option<Platform<'day>> {
        walking
            .filter(|snap| {
                day.leg(snap.index()).is_some_and(|leg| {
                    leg.length() - snap.fix().along() <= Nearness::AtStation.distance()
                })
            })
            .and_then(|snap| Self::ride_at(day, snap.index().incremented()))
            .map(|platform| Platform {
                walking,
                ..platform
            })
    }

    #[must_use]
    pub fn nearby(day: &'day Day, here: Coordinate, from: LegIndex) -> Option<Platform<'day>> {
        (usize::from(from)..day.legs().len())
            .filter_map(|position| Self::ride_at(day, LegIndex::new(position)))
            .find(|platform| {
                platform.leg.boarding().is_some_and(|boarding| {
                    here.distance_to(boarding) <= Nearness::AtStation.distance()
                })
            })
    }

    fn ride_at(day: &'day Day, ride: LegIndex) -> Option<Platform<'day>> {
        day.leg(ride)
            .filter(|leg| leg.mode() != Mode::Walk)
            .map(|leg| Platform {
                ride,
                leg,
                walking: None,
            })
    }

    #[must_use]
    pub fn ride(self) -> LegIndex {
        self.ride
    }

    #[must_use]
    pub fn phase(self, now: TripTime) -> Phase {
        let minutes = self.leg.time(End::Start).since_midnight() - now.time().since_midnight();
        if minutes >= Minutes::ZERO {
            Phase::Waiting { minutes }
        } else if -minutes <= RideSlack::Departure.for_mode(self.leg.mode()) {
            Phase::Overdue
        } else {
            Phase::Missed
        }
    }

    #[must_use]
    pub fn departing(self, here: Coordinate, now: TripTime) -> Option<Snap> {
        let station = Nearness::AtStation.distance();
        self.leg
            .line()
            .fix(here)
            .filter(|_| now.time() > self.leg.time(End::Start))
            .filter(|fix| fix.along() > station && fix.off_route() < station)
            .filter(|fix| {
                self.walking
                    .is_none_or(|walk| fix.off_route() < walk.fix().off_route())
            })
            .map(|fix| Snap::new(self.ride, fix))
    }

    #[must_use]
    pub fn late(self, now: TripTime) -> Minutes {
        if self.phase(now) == Phase::Missed {
            now.time().since_midnight() - self.leg.time(End::Start).since_midnight()
        } else {
            Minutes::ZERO
        }
    }
}
