use crate::clock::ClockTime;
use crate::day_number::DayNumber;
use crate::end::End;
use crate::itinerary::day::goal::{Goal, LoopStop};
use crate::itinerary::{Finding, Problem};
use crate::leg::{Leg, LegIndex};
use crate::times::PlanTime;

pub struct LoopCheck<'day> {
    day: DayNumber,
    legs: &'day [Leg],
    goal: &'day Goal,
}

impl<'day> LoopCheck<'day> {
    const LEAST_LOOP_LENGTH: usize = 2;
    const NEIGHBOUR_PAIR_LENGTH: usize = 2;

    #[must_use]
    pub fn new(day: DayNumber, legs: &'day [Leg], goal: &'day Goal) -> LoopCheck<'day> {
        LoopCheck { day, legs, goal }
    }

    #[must_use]
    pub fn findings(&self) -> Vec<Finding> {
        let stops = self.goal.purpose().stops();
        let short = (!stops.is_empty() && stops.len() < Self::LEAST_LOOP_LENGTH)
            .then(|| Finding::on_leg(self.day, self.goal.leg(End::Finish), Problem::LoopTooShort));
        let outside = stops
            .iter()
            .map(Self::leg_of)
            .filter(|leg| !self.goal.holds(*leg))
            .map(|leg| Finding::on_leg(self.day, leg, Problem::LoopOutsideGoal));
        let unordered = stops
            .windows(Self::NEIGHBOUR_PAIR_LENGTH)
            .filter(|pair| Self::leg_of(&pair[1]) <= Self::leg_of(&pair[0]))
            .map(|pair| Finding::on_leg(self.day, Self::leg_of(&pair[1]), Problem::LoopOutOfOrder));
        let late = stops.iter().filter_map(|stop| self.off_arrival(stop));
        short
            .into_iter()
            .chain(outside)
            .chain(unordered)
            .chain(late)
            .collect()
    }

    fn leg_of(stop: &LoopStop<'_>) -> LegIndex {
        *AsRef::<LegIndex>::as_ref(stop)
    }

    fn off_arrival(&self, stop: &LoopStop<'_>) -> Option<Finding> {
        let planned = *AsRef::<PlanTime>::as_ref(stop);
        let leg = self.legs.get(usize::from(Self::leg_of(stop)))?;
        (ClockTime::from(planned) != leg.time(End::Finish)).then(|| {
            Finding::on_leg(
                self.day,
                Self::leg_of(stop),
                Problem::LoopTimeOffLeg {
                    planned,
                    arrival: leg.listed(End::Finish),
                },
            )
        })
    }
}
