use guide::clock::{ClockTime, Minutes};

#[test]
fn times_round_trip_through_hours_and_minutes() {
    let time = ClockTime::try_from("07:05".to_owned()).map(ClockTime::since_midnight);
    assert_eq!(time.ok(), Some(Minutes::new(425)));
}

#[test]
fn impossible_times_are_rejected() {
    assert!(ClockTime::try_from("24:10".to_owned()).is_err());
}
