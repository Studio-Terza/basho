use crate::fixture::Fixture;
use crate::fixture::WrittenTime;
use guide::BufferState;
use guide::clock::Minutes;
use guide::clock::TripTime;
use guide::end::End;
use guide::geo::Distance;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::guidance::timeline::Halt;
use guide::leg::LegIndex;

const DAY_TWO: usize = 2;
const DAY_THREE: usize = 3;
const MILL_RACE_PATH: LegIndex = LegIndex::new(5);
const LATE: &str = "11:05";
const TEA_HOUSE_WALK: LegIndex = LegIndex::new(13);
const PAST_THE_POND: Distance = Distance::new(60.0);
const PAUSED_SINCE: &str = "13:45";
const NOW: &str = "13:48";
const STATION_WALK: LegIndex = LegIndex::new(0);
const OAKFORD_TRAIN: LegIndex = LegIndex::new(1);

#[test]
fn a_short_pause_just_after_a_finished_stop_does_not_restart_it() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let here = day
        .leg(TEA_HOUSE_WALK)
        .unwrap()
        .line()
        .point_along(PAST_THE_POND)
        .unwrap();
    let progress = LegProgress::confirmed(day.number(), TEA_HOUSE_WALK, Minutes::ZERO);
    let guidance = Guidance::new(day, progress, day.at_written(NOW), Some(here)).paused(Some(
        Halt::tracked(None, here, day.at_written(PAUSED_SINCE)),
    ));
    assert!(guidance.stop().is_none());
    assert!(
        guidance
            .buffer()
            .is_some_and(|buffer| buffer.spare() >= Minutes::ZERO)
    );
}

#[test]
fn a_walker_still_on_the_way_after_the_target_is_short() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let leg = day.leg(MILL_RACE_PATH).unwrap();
    let here = leg.line().point_along(leg.line().length() * 0.5);
    let progress = LegProgress::confirmed(day.number(), MILL_RACE_PATH, Minutes::ZERO);
    let state = Guidance::new(day, progress, day.at_written(LATE), here)
        .buffer()
        .map(|buffer| buffer.state());
    assert_eq!(state, Some(BufferState::Short));
}

#[test]
fn a_goal_that_ends_on_a_ride_shows_that_ride() {
    let trip = Fixture::bundled_days(&[DAY_TWO]);
    let day = trip.day(DAY_TWO);
    let start = day.leg(STATION_WALK).unwrap().time(End::Start);
    let guidance = Guidance::new(
        day,
        LegProgress::default(),
        TripTime::on_date_at(day.zone(), day.date(), start),
        None,
    );
    let ride = guidance
        .buffer_through(STATION_WALK, OAKFORD_TRAIN)
        .map(|buffer| buffer.deadline().leg());
    assert_eq!(ride, Some(OAKFORD_TRAIN));
}
