use guide::geo::Degrees;
use std::ops::Range;

use guide::geo::Coordinate;

const WEST_POINT: Coordinate = Coordinate::new(Degrees::new(38.0000), Degrees::new(-78.0000));
const EAST_POINT: Coordinate = Coordinate::new(Degrees::new(38.0500), Degrees::new(-77.9638));
const EXPECTED_KILOMETERS: Range<f64> = 6.0..6.6;

#[test]
fn west_point_to_east_point_is_about_six_kilometers() {
    let kilometers = WEST_POINT.distance_to(EAST_POINT).kilometers().amount();
    assert!(EXPECTED_KILOMETERS.contains(&kilometers), "{kilometers}");
}

#[test]
fn east_point_lies_north_east_of_west_point() {
    let bearing = WEST_POINT.bearing_to(EAST_POINT);
    assert_eq!(bearing.to_string(), "NE");
}
