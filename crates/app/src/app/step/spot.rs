use crate::app::Position;
use guide::day_number::DayNumber;
use guide::itinerary::{Day, Itinerary};
use map_plan::MapLevel;

use super::Step;
use super::goal_number::GoalNumber;
use super::step_number::StepNumber;
use super::toward::Toward;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spot {
    day: DayNumber,
    step: StepNumber,
}

impl Spot {
    #[must_use]
    pub const fn new(day: DayNumber, step: StepNumber) -> Spot {
        Spot { day, step }
    }

    #[must_use]
    pub fn entering(day: &Day, toward: Toward) -> Spot {
        toward.chosen(
            || {
                Spot::new(
                    day.number(),
                    StepNumber::from_position(Position::new(
                        Step::of_day(day).len().saturating_sub(1),
                    )),
                )
            },
            || Spot::new(day.number(), StepNumber::from_position(Position::new(0))),
        )
    }

    #[must_use]
    pub fn start_of_goal(day: &Day, goal: GoalNumber) -> Option<Spot> {
        let wanted = day.goals().get(usize::from(goal))?;
        Step::of_day(day)
            .into_iter()
            .find(|step| wanted.holds(step.index()))
            .map(|step| Spot::new(day.number(), step.number()))
    }

    #[must_use]
    pub fn day(self) -> DayNumber {
        self.day
    }

    #[must_use]
    pub fn step(self) -> StepNumber {
        self.step
    }

    #[must_use]
    pub fn stepped(self, toward: Toward, itinerary: &Itinerary) -> Option<Spot> {
        let count = itinerary
            .day_number(self.day)
            .map(|day| Step::of_day(day).len())?;
        let position = usize::from(self.step);
        toward
            .chosen(
                || position.checked_sub(1),
                || Some(position + 1).filter(|next| *next < count),
            )
            .map(|inside| Spot::new(self.day, StepNumber::from_position(Position::new(inside))))
            .or_else(|| self.across(itinerary.days(), toward))
    }

    #[must_use]
    pub fn stepped_across(
        self,
        level: MapLevel,
        toward: Toward,
        itinerary: &Itinerary,
    ) -> Option<Spot> {
        match level {
            MapLevel::Leg => self.stepped(toward, itinerary),
            MapLevel::Goal => itinerary
                .day_number(self.day)
                .zip(self.goal_beside(itinerary, toward))
                .and_then(|(day, goal)| Spot::start_of_goal(day, goal))
                .or_else(|| self.across(itinerary.days(), toward)),
            MapLevel::Day => self.across(itinerary.days(), toward),
        }
    }

    fn goal_beside(self, itinerary: &Itinerary, toward: Toward) -> Option<GoalNumber> {
        let day = itinerary.day_number(self.day)?;
        let steps = Step::of_day(day);
        let shown = steps.iter().find(|step| step.number() == self.step)?;
        let position = day
            .goals()
            .iter()
            .position(|goal| goal.holds(shown.index()))?;
        toward
            .chosen(
                || position.checked_sub(1),
                || Some(position + 1).filter(|next| *next < day.goals().len()),
            )
            .map(Position::new)
            .map(GoalNumber::from_position)
    }

    fn across(self, days: &[Day], toward: Toward) -> Option<Spot> {
        let index = days.iter().position(|day| day.number() == self.day)?;
        toward
            .chosen(
                || index.checked_sub(1).and_then(|before| days.get(before)),
                || days.get(index + 1),
            )
            .map(|day| Spot::entering(day, toward))
    }
}
