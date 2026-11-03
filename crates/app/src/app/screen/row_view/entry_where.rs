use std::fmt;

use guide::itinerary::Itinerary;
use wiki::Home;

use crate::app::screen::day_name::DayName;
use crate::app::step::{Step, StepNumber};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntryWhere {
    home: Home,
    place: HomePlace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HomePlace {
    Day,
    Goal,
    Step(StepNumber),
}

impl EntryWhere {
    #[must_use]
    pub fn new(home: Home, itinerary: &Itinerary) -> EntryWhere {
        let place = home.reached(
            HomePlace::Day,
            |_| HomePlace::Goal,
            |leg| {
                itinerary
                    .day_number(home.home_day())
                    .into_iter()
                    .flat_map(Step::of_day)
                    .find(|step| step.index() == leg)
                    .map_or(HomePlace::Day, |step| HomePlace::Step(step.number()))
            },
        );
        EntryWhere { home, place }
    }
}

impl fmt::Display for EntryWhere {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let day = DayName::new(self.home.home_day());
        match self.place {
            HomePlace::Day => write!(formatter, "{day}"),
            HomePlace::Goal => write!(formatter, "{day} goal"),
            HomePlace::Step(step) => write!(formatter, "{day} step {step}"),
        }
    }
}
