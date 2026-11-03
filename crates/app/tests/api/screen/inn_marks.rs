use guide::clock::Minutes;
use guide::itinerary::day::goal::{Goal, Purpose};

use basho::app::visit::Screen;

use crate::ui_check::{Drawing, Harness, Stage};

const MARK: &str = "class=\"unverified\"";
const HEAD: &str = "class=\"goal-head\"";
const ROW: &str = "class=\"goal-row\"";
const NAME_END: &str = "</strong>";
const DEADLINE: &str = "class=\"deadline\">";
const CHECK_IN: &str = " check in";

struct Named;

impl Named {
    fn marked(page: &str, opening: &str) -> Vec<bool> {
        page.split(opening)
            .skip(1)
            .filter_map(|rest| rest.split(NAME_END).next())
            .map(|name| name.contains(MARK))
            .collect()
    }

    fn at_inn(goal: &Goal) -> bool {
        matches!(goal.purpose(), Purpose::Inn)
    }
}

#[test]
fn an_unverified_inn_is_marked_in_the_goal_rows_of_today_and_the_menu() {
    let trip = Drawing::BuiltIn.itinerary();
    let days = trip.days();
    let pages: Vec<String> = days
        .iter()
        .map(|day| {
            Harness::page(
                &Stage::on_screen(day, Screen::Today)
                    .browsing_at(day, Stage::middle_of(day))
                    .with_menu(),
            )
        })
        .collect();
    let heads: Vec<bool> = pages
        .iter()
        .flat_map(|page| Named::marked(page, HEAD))
        .collect();
    let goals: Vec<bool> = days
        .iter()
        .flat_map(|day| day.goals().iter().map(Named::at_inn))
        .collect();
    assert!(goals.contains(&true) && goals.contains(&false));
    assert_eq!(heads, goals);
    assert!(pages.iter().all(|page| Named::marked(page, ROW) == goals));
}

#[test]
fn an_unverified_inn_is_marked_in_the_check_in_row() {
    let trip = Drawing::BuiltIn.itinerary();
    let stage = Stage::on_schedule(&trip, Minutes::new(20)).expect("a walk on schedule");
    let page = Harness::page(&stage);
    let places: Vec<&str> = page
        .split(DEADLINE)
        .skip(1)
        .filter_map(|rest| rest.split_once(CHECK_IN).map(|(place, _)| place))
        .collect();
    assert_ne!(places, Vec::<&str>::new());
    assert!(places.iter().all(|place| place.contains(MARK)));
}
