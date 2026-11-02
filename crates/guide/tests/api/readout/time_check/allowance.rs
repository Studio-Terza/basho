use crate::fixture::Fixture;
use guide::clock::Minutes;
use guide::end::End;
use guide::leg::LegIndex;
use guide::readout::time_check::allowance::Allowance;

const SAMPLE_DAY: usize = 3;
const MILL_RACE: LegIndex = LegIndex::new(5);
const STATION_ROAD: LegIndex = LegIndex::new(6);
const LOCAL_TRAIN: LegIndex = LegIndex::new(7);
const SETS_OFF: End = End::Start;
const ARRIVES: End = End::Finish;
const FLOOR: Minutes = Minutes::new(3);

#[test]
fn a_walk_allows_a_share_of_its_minutes_but_never_less_than_the_floor() {
    let trip = Fixture::bundled_days(&[SAMPLE_DAY]);
    let day = trip.day(SAMPLE_DAY);
    let long = day.leg(MILL_RACE).unwrap();
    let planned = long.time(SETS_OFF).minutes_until(long.time(ARRIVES));
    let share = Minutes::rounded(planned.amount() * 0.15).unwrap_or_default();
    assert!(share > FLOOR);
    assert_eq!(Allowance::for_leg(long), share);
    assert_eq!(Allowance::for_leg(day.leg(STATION_ROAD).unwrap()), FLOOR);
    assert_eq!(Allowance::for_leg(day.leg(LOCAL_TRAIN).unwrap()), FLOOR);
}
