use crate::clock::{Minutes, TripTime};
use crate::end::End;
use crate::geo::{Coordinate, RouteFix, Share};
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
        let leg = self.day.leg(ride).filter(|leg| self.under_way(leg))?;
        leg.line()
            .fix(self.here)
            .filter(|fix| fix.off_route() <= Nearness::ClearlyElsewhere.beside(leg.line()))
            .map(|fix| Snap::new(ride, fix))
    }

    #[must_use]
    pub fn upcoming(self, after: LegIndex) -> Option<Snap> {
        let next: Vec<LegIndex> = self
            .day
            .legs()
            .iter()
            .enumerate()
            .skip(usize::from(after))
            .skip_while(|(_, leg)| leg.mode() == Mode::Walk)
            .take_while(|(_, leg)| leg.mode() != Mode::Walk)
            .filter(|(_, leg)| self.under_way(leg))
            .map(|(index, _)| LegIndex::new(index))
            .collect();
        next.into_iter()
            .filter(|_| self.away_from_walks())
            .filter_map(|index| {
                let leg = self.day.leg(index)?;
                leg.line()
                    .fix(self.here)
                    .filter(|fix| fix.off_route() <= Nearness::OnRoute.beside(leg.line()))
                    .filter(|fix| self.on_schedule(leg, *fix))
                    .map(|fix| Snap::new(index, fix))
            })
            .min_by_key(|snap| snap.fix().off_route())
    }

    fn under_way(self, leg: &Leg) -> bool {
        let clock = self.now.minutes_into(self.day.date());
        let station = Nearness::AtStation.distance();
        leg.mode() != Mode::Walk
            && clock >= leg.time(End::Start).since_midnight()
            && clock
                <= leg.time(End::Finish).since_midnight() + RideSlack::Arrival.for_mode(leg.mode())
            && self.here.distance_to(leg.point(End::Finish)) > station
            && leg
                .boarding()
                .is_none_or(|boarding| self.here.distance_to(boarding) > station)
    }

    fn on_schedule(self, leg: &Leg, fix: RouteFix) -> bool {
        let clock = self.now.minutes_into(self.day.date());
        let start = leg.time(End::Start).since_midnight();
        let span = (leg.time(End::Finish).since_midnight() - start).max(Minutes::new(1));
        let share = |minutes: Minutes| Share::ratio((minutes - start).amount(), span.amount());
        let length = leg.line().length();
        let station = Nearness::AtStation.distance();
        fix.along() + station >= length * share(clock - RideSlack::Departure.for_mode(leg.mode()))
            && fix.along() <= length * share(clock + Self::CLOCK_STEP) + station
    }

    fn away_from_walks(self) -> bool {
        self.day
            .legs()
            .iter()
            .filter(|leg| leg.mode() == Mode::Walk)
            .all(|leg| {
                leg.line()
                    .fix(self.here)
                    .is_none_or(|fix| fix.off_route() > Nearness::OnRoute.beside(leg.line()))
            })
    }
}
