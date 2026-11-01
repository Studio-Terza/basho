mod day;

use crate::fixture::{DaySource, Fixture};

const ARCS: &str = "[[arcs]]\nnumber = 1\ntitle = [\"A → B\"]\nstart = \"A\"\ndays = [3]\n";
const DAY: &str = include_str!("fixture_day.toml");
const TIMED: &str = "minutes = 5";
const RUSHED: &str = "minutes = 1";
const THEN: &str = "Turn left.";
const WORDY: &str = "Turn left. ";
const REPEATS: usize = 12;
const EXPECTED: usize = 3;
const ROUTES: &str = r#"{"legs":{"1":{"segments":[{"no_mapped_path":{"via":[[140.0,38.0],[140.004,38.0]]}}]},"2":{"segments":[{"no_mapped_path":{"via":[[140.004,38.0],[140.012,38.0]]}}]}}}"#;

#[test]
fn a_fixture_day_loads_with_its_legs_lines_and_map() {
    let itinerary = Fixture::parse(ARCS, &[DaySource::new(DAY, ROUTES)]).expect("fixture");
    let legs = itinerary
        .days()
        .iter()
        .map(|day| day.legs().len())
        .sum::<usize>();
    assert_eq!(legs, 2);
}

#[test]
fn a_leg_that_is_too_fast_or_too_wordy_is_reported() {
    let broken = DAY
        .replace(TIMED, RUSHED)
        .replace(THEN, &WORDY.repeat(REPEATS));
    let findings = Fixture::inspect(ARCS, &[DaySource::new(&broken, ROUTES)]);
    assert_eq!(findings.len(), EXPECTED, "{findings:?}");
}
