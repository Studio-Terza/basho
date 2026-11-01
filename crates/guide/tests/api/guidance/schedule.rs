use crate::fixture::{DaySource, Fixture};
use guide::clock::CalendarDate;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::guidance::LegProgress;
use guide::guidance::Phase;
use guide::guidance::schedule::Schedule;
use guide::leg::LegIndex;

const ARCS: &str = "[[arcs]]\nnumber = 1\ntitle = [\"A → B\"]\nstart = \"A\"\ndays = [3]\n";
const DAY: &str = include_str!("../itinerary/fixture_day.toml");
const ROUTES: &str = r#"{"legs":{"1":{"segments":[{"no_mapped_path":{"via":[[140.0,38.0],[140.004,38.0]]}}]},"2":{"segments":[{"no_mapped_path":{"via":[[140.004,38.0],[140.012,38.0]]}}]}}}"#;
const DATE: &str = "2027-05-03";
const EARLIER: &str = "2027-04-23";
const LATER: &str = "08:16";
const EVENING: &str = "20:00";
const SECOND_LEG: usize = 1;
const TEN_LATE: i64 = 10;

#[test]
fn being_late_on_a_confirmed_leg_shifts_the_rest_of_the_timetable() {
    let trip = Fixture::parse(ARCS, &[DaySource::new(DAY, ROUTES)]).unwrap();
    let date = CalendarDate::try_from(DATE.to_owned()).unwrap();
    let now = TripTime::on_date_at(
        Fixture::zone(),
        date,
        ClockTime::try_from(LATER.to_owned()).unwrap(),
    );
    let day = trip.days().first().unwrap();
    let schedule = Schedule::estimate(
        day,
        LegProgress::confirmed(day.number(), LegIndex::default(), Minutes::new(TEN_LATE)),
        now,
    );
    assert_eq!(
        [
            usize::from(*AsRef::<LegIndex>::as_ref(&schedule)),
            usize::try_from(schedule.delay().unwrap().count()).unwrap()
        ],
        [SECOND_LEG, usize::try_from(TEN_LATE).unwrap()]
    );
}

#[test]
fn after_the_last_leg_the_day_is_done() {
    let trip = Fixture::parse(ARCS, &[DaySource::new(DAY, ROUTES)]).unwrap();
    let date = CalendarDate::try_from(DATE.to_owned()).unwrap();
    let now = TripTime::on_date_at(
        Fixture::zone(),
        date,
        ClockTime::try_from(EVENING.to_owned()).unwrap(),
    );
    assert_eq!(
        Schedule::estimate(trip.days().first().unwrap(), LegProgress::default(), now).phase(),
        Phase::Done
    );
}

#[test]
fn before_the_day_it_is_a_preview() {
    let trip = Fixture::parse(ARCS, &[DaySource::new(DAY, ROUTES)]).unwrap();
    let date = CalendarDate::try_from(EARLIER.to_owned()).unwrap();
    let now = TripTime::on_date_at(
        Fixture::zone(),
        date,
        ClockTime::try_from(EVENING.to_owned()).unwrap(),
    );
    assert_eq!(
        Schedule::estimate(trip.days().first().unwrap(), LegProgress::default(), now).phase(),
        Phase::Preview
    );
}
