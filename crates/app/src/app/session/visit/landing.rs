use guide::end::End;
use guide::itinerary::{Day, Itinerary};
use wiki::Home;

use crate::app::session::visit::scroll::Scroll;
use crate::app::step::{GoalNumber, Spot, Step, StepNumber, Toward};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Landing {
    #[default]
    Now,
    Held,
    Top,
    Goal(GoalNumber),
    Step(StepNumber),
}

impl Landing {
    #[must_use]
    pub fn of_home(home: Home, itinerary: &Itinerary) -> Landing {
        let day = itinerary.day_number(home.home_day());
        home.reached(
            Landing::Top,
            |first| {
                day.into_iter()
                    .flat_map(Day::goals)
                    .position(|goal| goal.leg(End::Start) == first)
                    .map_or(Landing::Top, |position| {
                        Landing::Goal(GoalNumber::from_position(position))
                    })
            },
            |leg| {
                day.into_iter()
                    .flat_map(Step::of_day)
                    .find(|step| step.index() == leg)
                    .map_or(Landing::Top, |step| Landing::Step(step.number()))
            },
        )
    }

    #[must_use]
    pub fn step(self, current: Option<StepNumber>) -> Option<StepNumber> {
        match self {
            Landing::Now => current,
            Landing::Step(step) => Some(step),
            Landing::Held | Landing::Top | Landing::Goal(_) => None,
        }
    }

    #[must_use]
    pub fn scrolled(self, scroll: Scroll) -> Landing {
        match (self, scroll) {
            (Landing::Now, Scroll::Hold) => Landing::Held,
            (Landing::Held, Scroll::Follow) => Landing::Now,
            (
                Landing::Now | Landing::Held | Landing::Top | Landing::Goal(_) | Landing::Step(_),
                _,
            ) => self,
        }
    }

    #[must_use]
    pub fn goal(self) -> Option<GoalNumber> {
        match self {
            Landing::Goal(goal) => Some(goal),
            Landing::Now | Landing::Held | Landing::Top | Landing::Step(_) => None,
        }
    }

    #[must_use]
    pub fn spot_in(self, day: &Day) -> Option<Spot> {
        match self {
            Landing::Now | Landing::Held => None,
            Landing::Top => Some(Spot::entering(day, Toward::Next)),
            Landing::Step(step) => Some(Spot::new(day.number(), step)),
            Landing::Goal(goal) => Spot::start_of_goal(day, goal),
        }
    }
}
