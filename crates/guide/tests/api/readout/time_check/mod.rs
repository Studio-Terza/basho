mod allowance;

use crate::fixture::Fixture;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Distance;
use guide::geo::Share;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::guidance::Sighting;
use guide::leg::LegIndex;
use guide::readout::Arrow;
use guide::readout::TimeCheck;
use guide::times::{FactTime, ShownTime};

const SAMPLE_DAY: usize = 3;
const MILL_RACE: LegIndex = LegIndex::new(5);
const STATION_ROAD: LegIndex = LegIndex::new(6);
const LATE_START: Minutes = Minutes::new(10);
const PAST_ALLOWANCE: Minutes = Minutes::new(5);
const HALFWAY: Share = Share::HALF;
const BRIEF: Minutes = Minutes::new(1);
const STARTS: End = End::Start;
const ENDS: End = End::Finish;

#[test]
fn a_leg_started_late_is_expected_that_much_later() {
    let trip = Fixture::bundled_days(&[SAMPLE_DAY]);
    let day = trip.day(SAMPLE_DAY);
    let road = day.leg(STATION_ROAD).unwrap();
    let kept = LegProgress::confirmed(day.number(), MILL_RACE, LATE_START);
    let now = TripTime::on_date_at(
        day.zone(),
        day.date(),
        road.time(STARTS).later_by(LATE_START),
    );
    let guidance = Guidance::new(day, kept, now, Some(road.point(STARTS)));
    assert_eq!(
        guidance.time_check(),
        Some(TimeCheck::Expected {
            by: ShownTime::off_by(road.listed(ENDS), LATE_START),
            left: road.time(STARTS).minutes_until(road.time(ENDS))
        })
    );
}

#[test]
fn still_on_the_leg_after_the_allowance_is_late() {
    let trip = Fixture::bundled_days(&[SAMPLE_DAY]);
    let day = trip.day(SAMPLE_DAY);
    let road = day.leg(STATION_ROAD).unwrap();
    let kept = LegProgress::confirmed(day.number(), STATION_ROAD, Minutes::ZERO);
    let now = TripTime::on_date_at(
        day.zone(),
        day.date(),
        road.time(ENDS).later_by(PAST_ALLOWANCE),
    );
    let here = road.line().point_along(road.length() * 0.5);
    assert_eq!(
        Guidance::new(day, kept, now, here).time_check(),
        Some(TimeCheck::Late {
            by: ShownTime::from(road.listed(ENDS)),
            over: PAST_ALLOWANCE
        })
    );
    assert!(road.length() > Distance::default());
}

#[test]
fn with_gps_lost_the_time_check_and_arrow_are_dated_and_the_arrow_goes_when_stale() {
    let trip = Fixture::bundled_days(&[SAMPLE_DAY]);
    let day = trip.day(SAMPLE_DAY);
    let road = day.leg(STATION_ROAD).unwrap();
    let along = road.length() * HALFWAY;
    let at = road.line().point_along(along).unwrap();
    let reached = road.time(STARTS).later_by(road.planned_after(along));
    let since = TripTime::on_date_at(day.zone(), day.date(), reached);
    let kept = LegProgress::confirmed(day.number(), STATION_ROAD, Minutes::ZERO);
    let quiet = |silence: Minutes| {
        let now = TripTime::on_date_at(day.zone(), day.date(), since.time().later_by(silence));
        Sighting::Stale { at, since }.guidance(day, kept, now)
    };
    let check = quiet(BRIEF).time_check().map(|check| check.to_string());
    assert!(check.is_some_and(|check| {
        check.ends_with(&format!("(from {})", FactTime::from_gps(since)))
    }));
    assert_eq!(
        quiet(BRIEF)
            .arrow()
            .and_then(Arrow::fix_time)
            .map(ClockTime::from),
        Some(since.time())
    );
    assert!(quiet(Arrow::STALE + BRIEF).arrow().is_none());
}
