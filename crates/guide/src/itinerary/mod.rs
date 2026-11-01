mod arc;
mod arc_number;
pub mod day;
mod day_number;
mod finding;
mod problem;

use crate::clock::CalendarDate;
use crate::trip::TripHead;

pub use arc::Arc;
pub use day::{Caption, Day, LegLines};
pub use day_number::DayNumber;
pub use finding::Finding;
pub use problem::Problem;

#[derive(Debug)]
pub struct Itinerary {
    head: TripHead,
    arcs: Vec<Arc>,
    days: Vec<Day>,
}

impl Itinerary {
    const DAY_PAIR_LENGTH: usize = 2;

    #[must_use]
    pub fn new(head: TripHead, arcs: Vec<Arc>, days: Vec<Day>) -> Itinerary {
        Itinerary { head, arcs, days }
    }

    #[must_use]
    pub fn head(&self) -> &TripHead {
        &self.head
    }

    #[must_use]
    pub fn arcs(&self) -> &[Arc] {
        &self.arcs
    }

    #[must_use]
    pub fn days(&self) -> &[Day] {
        &self.days
    }

    #[must_use]
    pub fn day_on(&self, date: CalendarDate) -> Option<&Day> {
        self.days.iter().find(|day| day.date() == date)
    }

    #[must_use]
    pub fn day_number(&self, number: DayNumber) -> Option<&Day> {
        self.days.iter().find(|day| day.number() == number)
    }

    #[must_use]
    pub fn arc_of(&self, day: &Day) -> Option<&Arc> {
        self.arcs.iter().find(|arc| arc.number() == day.arc())
    }

    #[must_use]
    pub fn days_of(&self, arc: &Arc) -> Vec<&Day> {
        arc.day_numbers()
            .iter()
            .filter_map(|number| self.day_number(*number))
            .collect()
    }
}

impl From<&Itinerary> for Vec<Finding> {
    fn from(itinerary: &Itinerary) -> Vec<Finding> {
        let membership = itinerary
            .days
            .iter()
            .filter(|day| {
                itinerary
                    .arc_of(day)
                    .is_none_or(|arc| !arc.day_numbers().contains(&day.number()))
            })
            .map(|day| Finding::on_day(day.number(), Problem::ArcMissesDay { arc: day.arc() }));
        let order = itinerary
            .days
            .windows(Itinerary::DAY_PAIR_LENGTH)
            .filter(|pair| !pair[1].follows(&pair[0]))
            .map(|pair| Finding::on_day(pair[1].number(), Problem::DayOutOfOrder));
        let repeated = itinerary
            .arcs
            .iter()
            .enumerate()
            .filter(|(index, arc)| {
                itinerary.arcs[..*index]
                    .iter()
                    .any(|earlier| earlier.number() == arc.number())
            })
            .map(|(_, arc)| Finding::whole(Problem::DuplicateArc { arc: arc.number() }));
        let empty = itinerary
            .days
            .is_empty()
            .then(|| Finding::whole(Problem::NoDays));
        itinerary
            .days
            .iter()
            .flat_map(Vec::<Finding>::from)
            .chain(membership)
            .chain(order)
            .chain(repeated)
            .chain(empty)
            .collect()
    }
}
