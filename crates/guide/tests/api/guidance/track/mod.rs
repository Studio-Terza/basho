use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::end::End;
use guide::geo::Distance;
use guide::leg::LegIndex;

const DAY_THREE: usize = 3;
const RIDGE_ROAD: LegIndex = LegIndex::new(4);
const MILL_RACE_PATH: LegIndex = LegIndex::new(5);
const KEPT_LATE: Minutes = Minutes::new(10);
const AWAY: Distance = Distance::new(200.0);
const LATER: Minutes = Minutes::new(30);

#[test]
fn lost_after_a_confirmed_step_the_arrow_points_back_to_it_and_the_spare_falls() {
    let trip = Fixture::bundled_days(&[DAY_THREE]);
    let day = trip.day(DAY_THREE);
    let walker = Walker::on_step(day, RIDGE_ROAD, KEPT_LATE);
    let here = walker.astray_from(RIDGE_ROAD, AWAY).unwrap();
    let ridge = day.leg(RIDGE_ROAD).unwrap();
    let line = ridge.line();
    let guidance = walker.fix(ridge.time(End::Start).later_by(KEPT_LATE), here);
    let start = day.leg(MILL_RACE_PATH).unwrap().time(End::Start);
    let lost = Walker::on_step(day, MILL_RACE_PATH, Minutes::ZERO);
    let off = lost.astray_from(MILL_RACE_PATH, AWAY).unwrap();
    let spare = |time: ClockTime| lost.fix(time, off).buffer().map(|buffer| buffer.spare());
    assert_eq!(guidance.leg_index(), RIDGE_ROAD);
    assert_eq!(
        guidance.arrow().map(|arrow| arrow.pointer().distance()),
        line.fix(here).map(|fix| Distance::from(fix.off_route()))
    );
    assert_eq!(
        spare(start.later_by(LATER)),
        spare(start).map(|first| first - LATER)
    );
}
