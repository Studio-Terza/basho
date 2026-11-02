use crate::fixture::{DaySource, Fixture};
use guide::times::PlanTime;

const ARCS: &str = "[[arcs]]\nnumber = 1\ntitle = [\"A → B\"]\nstart = \"A\"\ndays = [3]\n";
const DAY: &str = include_str!("../../fixture_day.toml");
const ROUTES: &str = r#"{"legs":{"1":{"segments":[{"no_mapped_path":{"via":[[140.0,38.0],[140.004,38.0]]}}]},"2":{"segments":[{"no_mapped_path":{"via":[[140.004,38.0],[140.012,38.0]]}}]}}}"#;
const AIMED: &str =
    "[[goals]]\nuntil = 1\nname = \"The ridge\"\nfor = \"landmark\"\nwhy = [\"See the road.\"]\n";
const MISAIMED: &str = "[[goals]]\nuntil = 2\nname = \"The door\"\nfor = \"sight\"\nwhy = [\"See it.\"]\n\n[[goals]]\nuntil = 9\nname = \"Nowhere\"\nfor = \"meal\"\nwhy = [\"Eat.\"]\n";
const MISSES: usize = 2;
const GARDEN_DAY: usize = 3;
const SIGHTS_OF_THE_GARDEN: usize = 3;

#[test]
fn goals_end_at_each_aim_and_the_inn_and_cover_every_leg_once() {
    let aimed = format!("{DAY}\n{AIMED}");
    let itinerary = Fixture::parse(ARCS, &[DaySource::new(&aimed, ROUTES)]).expect("fixture");
    let lengths = itinerary
        .days()
        .iter()
        .flat_map(|day| day.goals().iter().map(|goal| day.legs_toward(goal).len()))
        .collect::<Vec<usize>>();
    assert_eq!(lengths, [1, 1]);
}

#[test]
fn a_goal_on_the_inn_or_past_the_last_leg_is_reported() {
    let misaimed = format!("{DAY}\n{MISAIMED}");
    let findings = Fixture::inspect(ARCS, &[DaySource::new(&misaimed, ROUTES)]);
    assert_eq!(findings.len(), MISSES, "{findings:?}");
}

const FIRST_SIGHT: &str = "{ leg = 1, name = [\"The bridge\"], at = { plan = \"08:05\" } }";
const SECOND_SIGHT: &str = "{ leg = 2, name = [\"The inn\"], at = { plan = \"08:15\" } }";
const NIGHT_OPEN: &str = "[night]";
const LEGS_OPEN: &str = "[[legs]]";

struct Looped;

impl Looped {
    fn day_without_inn() -> String {
        let (head, rest) = DAY.split_once(NIGHT_OPEN).expect("the fixture has a night");
        let (_, legs) = rest.split_once(LEGS_OPEN).expect("the fixture has legs");
        format!("{head}{LEGS_OPEN}{legs}")
    }

    fn goal(sights: &[&str]) -> String {
        format!(
            "[[goals]]\nuntil = 2\nname = \"The town\"\nfor = {{ loop = {{ sights = [{}] }} }}\nwhy = [\"See both.\"]\n",
            sights.join(", ")
        )
    }

    fn findings(sights: &[&str]) -> usize {
        let day = format!("{}\n{}", Self::day_without_inn(), Self::goal(sights));
        Fixture::inspect(ARCS, &[DaySource::new(&day, ROUTES)]).len()
    }
}

#[test]
fn a_loop_goal_numbers_its_sights_in_order_with_their_planned_times() {
    let day = format!(
        "{}\n{}",
        Looped::day_without_inn(),
        Looped::goal(&[FIRST_SIGHT, SECOND_SIGHT])
    );
    let itinerary = Fixture::parse(ARCS, &[DaySource::new(&day, ROUTES)]).expect("fixture");
    let stops: Vec<String> = itinerary
        .days()
        .iter()
        .flat_map(|day| day.goals().iter().flat_map(|goal| goal.purpose().stops()))
        .map(|stop| format!("{} {}", stop.number(), AsRef::<PlanTime>::as_ref(&stop)))
        .collect();
    assert_eq!(stops, ["1 plan 8:05", "2 plan 8:15"]);
}

#[test]
fn a_loop_of_one_sight_in_the_wrong_order_or_at_the_wrong_time_is_reported() {
    let late = FIRST_SIGHT.replace("08:05", "08:06");
    assert_eq!(Looped::findings(&[FIRST_SIGHT, SECOND_SIGHT]), 0);
    assert_eq!(Looped::findings(&[FIRST_SIGHT]), 1);
    assert_eq!(Looped::findings(&[SECOND_SIGHT, FIRST_SIGHT]), 1);
    assert_eq!(Looped::findings(&[&late, SECOND_SIGHT]), 1);
}

#[test]
fn the_garden_is_one_loop_of_three_sights_whose_times_never_go_back() {
    let days = Fixture::bundled_days(&[GARDEN_DAY]);
    let stops = days
        .day(GARDEN_DAY)
        .goals()
        .iter()
        .flat_map(|goal| goal.purpose().stops())
        .map(|stop| *AsRef::<PlanTime>::as_ref(&stop))
        .collect::<Vec<PlanTime>>();
    assert_eq!(stops.len(), SIGHTS_OF_THE_GARDEN);
    assert!(stops.is_sorted());
}
