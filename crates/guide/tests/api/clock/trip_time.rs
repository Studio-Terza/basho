use crate::fixture::Fixture;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use guide::clock::CalendarDate;
use guide::clock::ClockTime;
use guide::clock::TripTime;

const NEW_YEAR: &str = "2027-01-01";
const LAST_DATE_OF_2026: &str = "2026-12-31";

#[test]
fn the_unix_epoch_is_nine_in_the_morning_at_plus_nine() {
    let epoch = TripTime::at_instant(UNIX_EPOCH, Fixture::zone());
    assert_eq!(
        (
            CalendarDate::from(epoch).to_string(),
            epoch.time().since_midnight().count()
        ),
        ("1970-01-01".to_owned(), 9 * 60)
    );
}

#[test]
fn the_same_instant_reads_by_the_zone_of_the_trip() {
    let west = "-05:00".parse().expect("an offset");
    let epoch = TripTime::at_instant(UNIX_EPOCH, west);
    assert_eq!(
        (
            CalendarDate::from(epoch).to_string(),
            epoch.time().since_midnight().count()
        ),
        ("1969-12-31".to_owned(), 19 * 60)
    );
}

#[test]
fn a_second_before_local_midnight_stays_in_the_old_year() {
    let date = CalendarDate::try_from(NEW_YEAR.to_owned()).unwrap();
    let midnight = TripTime::on_date_at(Fixture::zone(), date, ClockTime::at_hour(0));
    let before = TripTime::at_instant(
        SystemTime::from(midnight) - Duration::from_secs(1),
        midnight.zone(),
    );
    assert_eq!(before.minutes_since(midnight).count(), -1);
    assert_eq!(
        (
            CalendarDate::from(before).to_string(),
            before.time().since_midnight().count()
        ),
        (LAST_DATE_OF_2026.to_owned(), 24 * 60 - 1)
    );
}
