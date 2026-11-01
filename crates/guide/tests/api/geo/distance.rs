use guide::geo::Distance;

const SHORT: f64 = 347.0;
const LONG: f64 = 2_460.0;

#[test]
fn short_distances_show_rounded_meters_and_long_ones_kilometers() {
    assert_eq!(Distance::new(SHORT).to_string(), "350 m");
    assert_eq!(Distance::new(LONG).to_string(), "2.5 km");
}
