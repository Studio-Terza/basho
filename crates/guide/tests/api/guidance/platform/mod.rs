use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::end::End;
use guide::guidance::LegProgress;
use guide::guidance::Phase;
use guide::guidance::PositionSource;
use guide::leg::Leg;
use guide::leg::LegIndex;
use guide::pressing::Pressing;

const DAY_THREE: usize = 3;
const MILL_RACE_PATH: LegIndex = LegIndex::new(5);
const STATION_ROAD: LegIndex = LegIndex::new(6);
const TRAIN: LegIndex = LegIndex::new(7);
const EARLY: &str = "11:07";
const AFTER_DEPARTURE: &str = "11:25";
const ON_BOARD: &str = "11:20";

#[test]
fn a_walker_who_reaches_the_station_early_waits_for_the_train_and_is_not_late() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let arrival = ClockTime::try_from(EARLY.to_owned()).unwrap();
    let station = day.leg(TRAIN).and_then(Leg::boarding);
    let guidance = Walker::walked(day, STATION_ROAD, arrival).fix(arrival, station.unwrap());
    assert_eq!(guidance.leg_index(), TRAIN);
    assert_eq!(guidance.phase().waiting_minutes(), Some(Minutes::new(10)));
    assert_eq!(
        guidance.progress().map(LegProgress::offset),
        Some(Minutes::ZERO)
    );
}

#[test]
fn a_walker_with_gps_history_still_at_the_station_after_departure_missed_it() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let arrival = ClockTime::try_from(EARLY.to_owned()).unwrap();
    let later = ClockTime::try_from(AFTER_DEPARTURE.to_owned()).unwrap();
    let station = day.leg(TRAIN).and_then(Leg::boarding).unwrap();
    let guidance = Walker::walked(day, STATION_ROAD, arrival)
        .stepped(arrival, station)
        .fix(later, station);
    assert_eq!(guidance.missed_ride(), Some(TRAIN));
    assert!(
        guidance
            .plans(STATION_ROAD)
            .iter()
            .any(|plan| plan.leg() == TRAIN && plan.pressing() == Pressing::Now)
    );
}

#[test]
fn a_fix_on_the_train_line_is_riding_even_when_the_timetable_says_walking() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let ride = day.leg(TRAIN).unwrap();
    let walked = day.leg(MILL_RACE_PATH).unwrap().time(End::Finish);
    let on_board = ride.line().point_along(ride.line().length() * 0.3);
    let guidance = Walker::walked(day, MILL_RACE_PATH, walked).fix(
        ClockTime::try_from(ON_BOARD.to_owned()).unwrap(),
        on_board.unwrap(),
    );
    assert_eq!(guidance.leg_index(), TRAIN);
    assert!(guidance.source() == PositionSource::Gps && guidance.off_route().is_none());
    assert_eq!(guidance.phase(), Phase::Moving);
}
