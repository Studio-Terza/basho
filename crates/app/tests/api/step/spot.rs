use guide::itinerary::{Day, DayNumber};

use map_plan::MapLevel;

use crate::ui_check::Drawing;
use basho::app::step::GoalNumber;
use basho::app::step::Spot;
use basho::app::step::StepNumber;
use basho::app::step::Toward;

const LAST_DAY: DayNumber = DayNumber::new(4);
const GOAL_DAY: DayNumber = DayNumber::new(3);
const INSIDE: Spot = Spot::new(LAST_DAY, StepNumber::from_position(1));
const AFTER: Spot = Spot::new(LAST_DAY, StepNumber::from_position(2));

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

#[test]
fn stepping_stops_at_the_ends_of_the_trip() {
    let trip = Drawing::BuiltIn.itinerary();
    let first = trip.days().first().map(Ends::first);
    let last = trip.days().last().map(Ends::last);
    assert_eq!(
        [
            first.and_then(|spot| spot.stepped(Toward::Previous, &trip)),
            last.and_then(|spot| spot.stepped(Toward::Next, &trip)),
            INSIDE.stepped(Toward::Next, &trip),
        ],
        [None, None, Some(AFTER)]
    );
}

#[test]
fn a_wider_map_level_steps_by_goal_and_by_day() {
    let trip = Drawing::BuiltIn.itinerary();
    let day = trip.day_number(GOAL_DAY).expect("the day is in the trip");
    let first = Ends::first(day);
    let next_goal = Spot::start_of_goal(day, GoalNumber::from_position(1));
    let next_day = trip.days().iter().find(|later| later.number() > GOAL_DAY);
    assert_eq!(
        first.stepped_across(MapLevel::Goal, Toward::Next, &trip),
        next_goal
    );
    assert_eq!(
        first.stepped_across(MapLevel::Day, Toward::Next, &trip),
        next_day.map(Ends::first)
    );
}
