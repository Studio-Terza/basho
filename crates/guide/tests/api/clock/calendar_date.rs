use guide::clock::CalendarDate;

const WALK_DAYS: [[&str; 2]; 3] = [
    ["2027-05-03", "Mon May 3"],
    ["2027-05-04", "Tue May 4"],
    ["2027-05-09", "Sun May 9"],
];

#[test]
fn dates_round_trip_through_text_across_a_leap_day() {
    let round_trip = ["2024-02-29", "2027-05-03"].map(|text| {
        CalendarDate::try_from(text.to_owned())
            .map(|date| date.to_string())
            .ok()
    });
    assert_eq!(
        round_trip,
        [Some("2024-02-29".to_owned()), Some("2027-05-03".to_owned())]
    );
}

#[test]
fn written_dates_parse_and_impossible_ones_fail() {
    assert!(CalendarDate::try_from("2027-05-03".to_owned()).is_ok());
    assert!(CalendarDate::try_from("2026-13-01".to_owned()).is_err());
    assert!(CalendarDate::try_from("2026-02-31".to_owned()).is_err());
}

#[test]
fn walk_days_name_their_weekday_and_month() {
    let named = WALK_DAYS.map(|[written, _]| {
        CalendarDate::try_from(written.to_owned())
            .map(|date| date.short().to_string())
            .ok()
    });
    assert_eq!(named, WALK_DAYS.map(|[_, short]| Some(short.to_owned())));
}
