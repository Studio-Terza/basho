use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::clock::ClockTime;
use guide::clock::Minutes;
use guide::end::End;
use guide::geo::Degrees;
use guide::geo::Distance;
use guide::geo::Share;
use guide::leg::LegIndex;
use guide::readout::Arrow;
use guide::readout::TimeCheck;

const HILL_DAY: usize = 3;
const CLIMB: LegIndex = LegIndex::new(2);
const DESCENT: LegIndex = LegIndex::new(3);
const AT_THE_FOOT: Distance = Distance::new(30.0);
const ON_THE_WAY_DOWN: [Distance; 4] = [
    Distance::new(100.0),
    Distance::new(250.0),
    Distance::new(400.0),
    Distance::new(550.0),
];
const DAWN: usize = 1;
const LEAVING: Distance = Distance::new(10.0);
const RIGHT_ANGLE: Degrees = Degrees::new(90.0);
const MORNINGS: [usize; 2] = [1, 3];

#[test]
fn reopened_on_the_way_down_the_walker_is_on_the_descent_and_not_late() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    let foot = day.leg(CLIMB).unwrap().line().point_along(AT_THE_FOOT);
    let walker = Walker::fresh(day).stepped(
        ClockTime::try_from("09:26".to_owned()).unwrap(),
        foot.unwrap(),
    );
    let later = ClockTime::try_from("10:12".to_owned()).unwrap();
    let line = day.leg(DESCENT).unwrap().line();
    let wrong: Vec<String> = ON_THE_WAY_DOWN
        .iter()
        .map(|along| walker.fix(later, line.point_along(*along).unwrap()))
        .filter(|guidance| {
            guidance.leg_index() != DESCENT
                || guidance.time_check().and_then(TimeCheck::late_by).is_some()
        })
        .map(|guidance| format!("{} {:?}", guidance.leg_index(), guidance.time_check()))
        .collect();
    assert_eq!(wrong, Vec::<String>::new());
}

#[test]
fn leaving_the_hotel_late_before_dawn_is_the_first_step_and_points_down_it() {
    let trip = Fixture::bundled_days(&[DAWN]);
    let day = trip.day(DAWN);
    let line = day.leg(LegIndex::default()).unwrap().line();
    let here = line.point_along(LEAVING).unwrap();
    let ahead = here.bearing_to(line.point_along(LEAVING + Arrow::LOOK_AHEAD).unwrap());
    let guidance = Walker::fresh(day).fix(ClockTime::try_from("05:45".to_owned()).unwrap(), here);
    assert_eq!(guidance.leg_index(), LegIndex::default());
    assert!(guidance.arrow().unwrap().pointer().bearing().gap_to(ahead) < RIGHT_ANGLE.degrees());
}

#[test]
fn waiting_at_the_inn_before_leaving_is_not_late_once_the_walk_starts_on_time() {
    let trip = Fixture::bundled_days(&MORNINGS);
    let late: Vec<String> = MORNINGS
        .iter()
        .filter_map(|number| {
            let day = trip.day(*number);
            let first = day.leg(LegIndex::default()).unwrap();
            let dawn = ClockTime::try_from("06:30".to_owned()).unwrap();
            let half = first.length() * Share::HALF;
            Walker::fresh(day)
                .stepped(dawn, first.point(End::Start))
                .strolled(LegIndex::default(), half, Minutes::ZERO)
                .fix_along(LegIndex::default(), half, Minutes::ZERO)
                .time_check()
                .and_then(TimeCheck::late_by)
                .map(|over| format!("day {number}: LATE {over}"))
        })
        .collect();
    assert_eq!(late, Vec::<String>::new());
}
