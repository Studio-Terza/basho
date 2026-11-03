use std::time::Instant;

use guide::plane::Pixels;
use guide::plane::PlanePoint;
use layout::scale::Viewport;

use basho::app::session::gesture::TouchSample;
use basho::app::session::gesture::stroke::Stroke;
use basho::app::step::Toward;

struct Drag;

impl Drag {
    fn swipes(moves: &[[f64; 2]]) -> Vec<Option<Toward>> {
        let start = Instant::now();
        let origin = TouchSample::new(
            PlanePoint::new(Pixels::new(215.0), Pixels::new(500.0)),
            start,
        );
        moves
            .iter()
            .map(|[across, down]| {
                let end = PlanePoint::new(
                    origin.point()[guide::plane::Axis::Across] + Pixels::new(*across),
                    origin.point()[guide::plane::Axis::Down] + Pixels::new(*down),
                );
                Stroke::between(origin, TouchSample::new(end, start)).swipe(Viewport::default())
            })
            .collect()
    }
}

const LONG_LEVEL: [[f64; 2]; 3] = [[-160.0, 0.0], [160.0, 12.0], [-90.0, -30.0]];
const NOT_SWIPES: [[f64; 2]; 5] = [
    [0.0, 0.0],
    [-30.0, 4.0],
    [60.0, -8.0],
    [-150.0, 120.0],
    [10.0, 300.0],
];

#[test]
fn a_long_level_swipe_left_goes_next_and_right_goes_back() {
    assert_eq!(
        Drag::swipes(&LONG_LEVEL),
        [
            Some(Toward::Next),
            Some(Toward::Previous),
            Some(Toward::Next)
        ]
    );
}

#[test]
fn taps_wiggles_short_drags_diagonals_and_scrolls_are_not_swipes() {
    assert_eq!(Drag::swipes(&NOT_SWIPES), [None; 5]);
}
