use crate::clock::{Minutes, TripTime};
use crate::end::End;
use crate::geo::{Coordinate, Distance, RouteFix, Share};
use crate::guidance::leg_progress::LegProgress;
use crate::guidance::nearness::Nearness;
use crate::guidance::ride_slack::RideSlack;
use crate::guidance::snap::Snap;
use crate::itinerary::Day;
use crate::leg::{Leg, LegIndex, Mode};

#[derive(Clone, Copy, Debug)]
pub struct RideSearch<'day> {
    day: &'day Day,
    here: Coordinate,
    now: TripTime,
}

impl<'day> RideSearch<'day> {
    const CLOCK_STEP: Minutes = Minutes::new(1);

    #[must_use]
    pub fn new(day: &'day Day, here: Coordinate, now: TripTime) -> RideSearch<'day> {
        RideSearch { day, here, now }
    }

    #[must_use]
    pub fn riding_on(self, kept: LegProgress) -> Option<Snap> {
        let ride = kept.leg();
        let leg = self.under_way(ride)?;
        leg.line()
            .fix(self.here)
            .filter(|fix| fix.off_route() <= Nearness::ClearlyElsewhere.beside(leg.line()))
            .map(|fix| Snap::new(ride, fix))
    }

    #[must_use]
    pub fn upcoming(self, after: LegIndex) -> Option<Snap> {
        match self.on_walk() {
            Some(_) => None,
            None => self.next_ride(after),
        }
    }

    fn next_ride(self, after: LegIndex) -> Option<Snap> {
        self.day
            .legs()
            .iter()
            .enumerate()
            .skip(usize::from(after))
            .skip_while(|(_, leg)| leg.mode() == Mode::Walk)
            .take_while(|(_, leg)| leg.mode() != Mode::Walk)
            .map(|(index, _)| LegIndex::new(index))
            .filter_map(|index| self.under_way(index).map(|leg| (index, leg)))
            .filter_map(|(index, leg)| {
                let clock = self.now.minutes_into(self.day.date());
                let station = Nearness::AtStation.distance();
                leg.line()
                    .fix(self.here)
                    .filter(|fix| fix.off_route() <= Nearness::OnRoute.beside(leg.line()))
                    .filter(|fix| {
                        fix.along() + station
                            >= Self::timetabled(
                                leg,
                                clock - RideSlack::Departure.for_mode(leg.mode()),
                            )
                    })
                    .filter(|fix| {
                        fix.along() <= Self::timetabled(leg, clock + Self::CLOCK_STEP) + station
                    })
                    .map(|fix| Snap::new(index, fix))
            })
            .min_by_key(|snap| snap.fix().off_route())
    }

    fn under_way(self, ride: LegIndex) -> Option<&'day Leg> {
        let clock = self.now.minutes_into(self.day.date());
        let station = Nearness::AtStation.distance();
        self.day
            .leg(ride)
            .filter(|leg| leg.mode() != Mode::Walk)
            .filter(|leg| clock >= leg.time(End::Start).since_midnight())
            .filter(|leg| {
                clock
                    <= leg.time(End::Finish).since_midnight()
                        + RideSlack::Arrival.for_mode(leg.mode())
            })
            .filter(|leg| self.here.distance_to(leg.point(End::Finish)) > station)
            .filter(|leg| {
                leg.boarding()
                    .is_none_or(|boarding| self.here.distance_to(boarding) > station)
            })
    }

    fn timetabled(leg: &Leg, clock: Minutes) -> Distance {
        let start = leg.time(End::Start).since_midnight();
        let span = (leg.time(End::Finish).since_midnight() - start).max(Minutes::new(1));
        leg.line().length() * Share::ratio((clock - start).amount(), span.amount())
    }

    fn on_walk(self) -> Option<RouteFix> {
        self.day
            .legs()
            .iter()
            .filter(|leg| leg.mode() == Mode::Walk)
            .find_map(|leg| {
                leg.line()
                    .fix(self.here)
                    .filter(|fix| fix.off_route() <= Nearness::OnRoute.beside(leg.line()))
            })
    }
}
