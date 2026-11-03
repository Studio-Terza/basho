mod cues;
mod deadlines;
mod geometry;
mod leg_rule;
mod lengths;
mod stops;
mod timing;
use crate::day_number::DayNumber;
use crate::itinerary::Finding;
use crate::leg::{Leg, LegIndex};
use cues::Cues;
use deadlines::Deadlines;
use geometry::Geometry;
use leg_rule::LegRule;
use lengths::Lengths;
use stops::Stops;
use timing::Timing;

pub struct LegCheck<'leg> {
    leg: &'leg Leg,
    day: DayNumber,
    index: LegIndex,
}

impl<'leg> LegCheck<'leg> {
    #[must_use]
    pub fn new(leg: &'leg Leg, day: DayNumber, index: LegIndex) -> LegCheck<'leg> {
        LegCheck { leg, day, index }
    }

    #[must_use]
    pub fn findings(&self) -> Vec<Finding> {
        let rules: &[&dyn LegRule] = &[&Lengths, &Geometry, &Timing, &Stops, &Cues, &Deadlines];
        rules
            .iter()
            .flat_map(|rule| rule.problems(self.leg))
            .map(|problem| Finding::on_leg(self.day, self.index, problem))
            .collect()
    }
}
