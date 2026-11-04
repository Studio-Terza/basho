use super::departure::Departure;
use crate::fixture::Fixture;
use crate::fixture::WrittenTime;
use guide::BufferState;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::geo::Coordinate;
use guide::geo::Distance;
use guide::guidance::Guidance;
use guide::guidance::LegProgress;
use guide::guidance::timeline::Halt;
use guide::leg::LegIndex;

const DAY_THREE: usize = 3;
const DAY_FOUR: usize = 4;
const LUNCH_WALK: LegIndex = LegIndex::new(11);
const ARRIVED_LATE: &str = "12:55";
const EATING: &str = "13:05";
const LATE_SPARE: Minutes = Minutes::new(18);
const DEPARTS: &str = "11:17";
const GONE: &str = "11:18";
const BACKUP_LEAVES: &str = "12:17";
const SHINKANSEN: LegIndex = LegIndex::new(8);
const ONE_LATE: &str = "17:56";
const SHORT_OF_THE_END: Distance = Distance::new(3000.0);

#[test]
fn arriving_late_at_lunch_keeps_the_lateness_while_eating() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let spot = Coordinate::from(day.leg(LUNCH_WALK).unwrap().along().last().unwrap());
    let progress = LegProgress::confirmed(day.number(), LUNCH_WALK, Minutes::ZERO);
    let halt = Halt::tracked(None, spot, day.at_written(ARRIVED_LATE));
    let guidance =
        Guidance::new(day, progress, day.at_written(EATING), Some(spot)).paused(Some(halt));
    assert_eq!(
        guidance.buffer().map(|buffer| buffer.spare()),
        Some(LATE_SPARE)
    );
}

#[test]
fn after_the_train_leaves_the_station_waits_for_the_backup() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let train = Departure::of(day, DEPARTS);
    let station = day.leg(train).unwrap().boarding();
    let progress = LegProgress::confirmed(day.number(), train.previous().unwrap(), Minutes::ZERO);
    let buffer = Guidance::new(day, progress, day.at_written(GONE), station)
        .buffer()
        .unwrap();
    assert_eq!(
        buffer.deadline().event(),
        ClockTime::try_from(BACKUP_LEAVES.to_owned()).unwrap()
    );
    assert_eq!(buffer.state(), BufferState::Waiting);
}

#[test]
fn a_planned_train_a_minute_late_is_not_taken_for_the_backup() {
    let trip = Fixture::bundled_days(&[DAY_FOUR]);
    let day = trip.day(DAY_FOUR);
    let shinkansen = day.leg(SHINKANSEN).unwrap();
    let here = shinkansen
        .line()
        .point_along(shinkansen.line().length() - SHORT_OF_THE_END);
    let progress = LegProgress::confirmed(day.number(), SHINKANSEN, Minutes::ZERO);
    let state = Guidance::new(day, progress, day.at_written(ONE_LATE), here)
        .buffer()
        .map(|buffer| buffer.state());
    assert_ne!(state, Some(BufferState::Short));
}
