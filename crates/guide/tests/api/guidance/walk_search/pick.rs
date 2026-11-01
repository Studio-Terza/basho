use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::clock::ClockTime;
use guide::geo::Degrees;
use guide::geo::Distance;
use guide::leg::LegIndex;
use guide::readout::Arrow;

const DAWN: usize = 1;
const LEAVING: Distance = Distance::new(10.0);
const RIGHT_ANGLE: Degrees = Degrees::new(90.0);

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
