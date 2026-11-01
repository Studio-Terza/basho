use crate::fixture::Fixture;
use guide::clock::CalendarDate;
use guide::clock::ClockTime;
use guide::clock::Sunset;
use guide::geo::Coordinate;
use guide::geo::Degrees;

const TOKYO: Coordinate = Coordinate::new(Degrees::new(35.6895), Degrees::new(139.6917));
const DATE: &str = "2027-06-21";
const NAOJ_SUNSET: &str = "19:00";
const SLACK_MINUTES: i64 = 2;

#[test]
fn sunset_in_tokyo_matches_the_observatory_within_two_minutes() {
    let date = CalendarDate::try_from(DATE.to_owned()).unwrap();
    let expected = ClockTime::try_from(NAOJ_SUNSET.to_owned()).unwrap();
    let set = Sunset::new(date, TOKYO, Fixture::zone()).time().unwrap();
    assert!(expected.minutes_until(set).count().abs() <= SLACK_MINUTES);
}
