use guide::geo::Bearing;
use guide::geo::bearing::compass_point::CompassPoint;

const JUST_WEST_OF_NORTH: f64 = 350.0;
const SOUTH_SOUTH_WEST: f64 = 205.0;
const SOUTH_EAST: f64 = 140.0;

#[test]
fn bearings_round_to_the_nearest_of_eight_points() {
    assert_eq!(
        Bearing::new(JUST_WEST_OF_NORTH).compass_point(),
        CompassPoint::N
    );
    assert_eq!(
        Bearing::new(SOUTH_SOUTH_WEST).compass_point(),
        CompassPoint::SW
    );
}

#[test]
fn a_point_spells_out_its_full_direction_in_words() {
    assert_eq!(
        Bearing::new(SOUTH_EAST).compass_point().words().to_string(),
        "SOUTH-EAST"
    );
}
