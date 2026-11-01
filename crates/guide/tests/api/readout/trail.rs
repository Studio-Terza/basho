use guide::geo::Coordinate;
use guide::geo::Degrees;
use guide::geo::Distance;
use guide::readout::Trail;

const START: Coordinate = Coordinate::new(Degrees::new(38.0), Degrees::new(140.0));
const STEP: Distance = Distance::new(3.0);
const STRIDE: Distance = Distance::new(20.0);

#[test]
fn fixes_closer_than_the_spacing_to_the_last_kept_one_are_dropped() {
    let trail: Trail = (0..5)
        .map(|step| START.offset_by(STEP * f64::from(step), Distance::default()))
        .collect();
    assert_eq!(trail.points().len(), 2);
}

#[test]
fn the_trail_keeps_only_the_most_recent_fixes() {
    let fixes = u32::try_from(Trail::CAPACITY * 2).unwrap();
    let trail: Trail = (0..fixes)
        .map(|step| START.offset_by(STRIDE * f64::from(step), Distance::default()))
        .collect();
    assert_eq!(trail.points().len(), Trail::CAPACITY);
    assert_eq!(
        trail.points().first().copied(),
        Some(START.offset_by(STRIDE * f64::from(fixes / 2), Distance::default()))
    );
}
