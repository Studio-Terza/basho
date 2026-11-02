mod pick;
mod stride;

use crate::fixture::Fixture;
use std::ops::Range;

use crate::walker::Walker;
use guide::clock::Minutes;
use guide::end::End;
use guide::geo::Degrees;
use guide::geo::Distance;
use guide::leg::LegIndex;
use guide::readout::Arrow;

const HILL_DAY: usize = 3;
const LATE: Minutes = Minutes::new(45);
const SETTLED: Distance = Distance::new(150.0);
const EXACT: Distance = Distance::new(0.0);
const OUT_AND_BACK: [LegIndex; 2] = [LegIndex::new(2), LegIndex::new(3)];
const LANE: LegIndex = LegIndex::new(0);
const PARALLEL_ROAD: LegIndex = LegIndex::new(5);
const STRAY: Range<f64> = 170.0..260.0;
const CLIMB: LegIndex = LegIndex::new(2);
const WALKED: Distance = Distance::new(400.0);
const JUMP_SHORT: Distance = Distance::new(40.0);
const RIGHT_ANGLE: Degrees = Degrees::new(90.0);

#[test]
fn walking_an_out_and_back_step_late_keeps_the_arrow_on_the_pass_being_walked() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    let wrong: Vec<String> = OUT_AND_BACK
        .iter()
        .flat_map(|walk| {
            let kept = Walker::on_step(day, *walk, LATE).wrong_turns(*walk, LATE, EXACT, EXACT);
            kept.into_iter()
                .chain(Walker::fresh(day).wrong_turns(*walk, LATE, EXACT, SETTLED))
        })
        .collect();
    assert_eq!(wrong, Vec::<String>::new());
}

#[test]
fn a_wrong_road_beside_another_step_is_off_route_for_the_step_being_walked() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    let guidance = Walker::fresh(day)
        .strayed(LANE, PARALLEL_ROAD, &STRAY)
        .unwrap();
    assert_eq!(guidance.leg_index(), LANE);
    assert!(guidance.off_route().is_some());
}

#[test]
fn one_jumpy_fix_ahead_does_not_move_the_walker_onto_the_way_back() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    let leg = day.leg(CLIMB).unwrap();
    let time = leg.time(End::Start).later_by(leg.planned_after(WALKED));
    let jumped = leg.line().point_along(leg.length() - JUMP_SHORT).unwrap();
    let next = leg.line().point_along(WALKED + Arrow::LOOK_AHEAD).unwrap();
    let guidance = Walker::fresh(day)
        .strolled(CLIMB, WALKED, Minutes::ZERO)
        .stepped(time, jumped)
        .fix(time, leg.line().point_along(WALKED).unwrap());
    let ahead = leg.line().point_along(WALKED).unwrap().bearing_to(next);
    assert_eq!(guidance.leg_index(), CLIMB);
    assert!(guidance.arrow().unwrap().pointer().bearing().gap_to(ahead) < RIGHT_ANGLE.degrees());
}
