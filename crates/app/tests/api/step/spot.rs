use guide::itinerary::{Day, DayNumber};

use crate::ui_check::Drawing;
use basho::app::step::Spot;
use basho::app::step::Toward;

struct Ends;

impl Ends {
    fn first(day: &Day) -> Spot {
        Spot::entering(day, Toward::Next)
    }

    fn last(day: &Day) -> Spot {
        Spot::entering(day, Toward::Previous)
    }
}

#[test]
fn stepping_crosses_into_the_next_and_previous_day() {
    let trip = Drawing::BuiltIn.itinerary();
    let days = trip.days();
    let broken: Vec<DayNumber> = days
        .iter()
        .zip(days.iter().skip(1))
        .filter(|(day, next)| {
            Ends::last(day).stepped(Toward::Next, &trip) != Some(Ends::first(next))
                || Ends::first(next).stepped(Toward::Previous, &trip) != Some(Ends::last(day))
        })
        .map(|(day, _)| day.number())
        .collect();
    assert_eq!(broken, Vec::new());
}
