use guide::geo::Degrees;
use std::ops::Range;

use guide::geo::Coordinate;
use guide::geo::Distance;
use guide::geo::RouteLine;

const WEST: Coordinate = Coordinate::new(Degrees::new(38.0), Degrees::new(140.0));
const EAST: Coordinate = Coordinate::new(Degrees::new(38.0), Degrees::new(140.01));
const NORTH_OF_MIDDLE: Coordinate = Coordinate::new(Degrees::new(38.001), Degrees::new(140.005));
const FAR: Coordinate = Coordinate::new(Degrees::new(38.05), Degrees::new(140.01));
const OFF_ROUTE: Range<f64> = 105.0..117.0;
const ALONG: Range<f64> = 430.0..445.0;

#[test]
fn a_point_beside_a_line_snaps_to_its_middle() {
    let fix = RouteLine::new(vec![WEST, EAST])
        .fix(NORTH_OF_MIDDLE)
        .expect("fix");
    assert!(
        OFF_ROUTE.contains(&Distance::from(fix.off_route()).meters()),
        "{fix:?}"
    );
    assert!(ALONG.contains(&fix.along().meters()), "{fix:?}");
}

#[test]
fn a_line_with_a_multi_kilometer_jump_is_a_sketch() {
    assert!(RouteLine::new(vec![WEST, EAST]).is_sketch().is_none());
    assert!(RouteLine::new(vec![WEST, EAST, FAR]).is_sketch().is_some());
}

#[test]
fn halfway_along_a_line_is_its_midpoint() {
    let line = RouteLine::new(vec![WEST, EAST]);
    let halfway = line.point_along(Distance::new(line.length().meters() / 2.0));
    assert!(
        halfway.is_some_and(|point| point.distance_to(NORTH_OF_MIDDLE).meters() < OFF_ROUTE.end)
    );
}
