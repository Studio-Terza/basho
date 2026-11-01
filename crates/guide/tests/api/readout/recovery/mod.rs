mod day_catch;

use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::geo::Distance;
use rayon::prelude::*;

const SIDES: [Distance; 2] = [Distance::new(-300.0), Distance::new(300.0)];

#[test]
fn off_the_route_every_catching_feature_shown_lies_toward_the_route() {
    let trip = Fixture::bundled().unwrap();
    let misleading: Vec<String> = trip
        .days()
        .par_iter()
        .flat_map_iter(|day| SIDES.map(|aside| Walker::misleading_sights(day, aside)))
        .flatten()
        .collect();
    assert_eq!(misleading, Vec::<String>::new());
}
