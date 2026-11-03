mod stroke;

use std::time::{Duration, Instant};

use guide::plane::Pixels;
use guide::plane::PlanePoint;

use layout::scale::Viewport;

use basho::app::session::gesture::Gesture;
use basho::app::session::gesture::Origin;
use basho::app::session::gesture::Outcome;
use basho::app::session::gesture::TouchSample;
use basho::app::step::Toward;

struct Swipe;

impl Swipe {
    fn ending(held: Option<Origin>, points: &[[f64; 3]]) -> Outcome {
        let start = Instant::now();
        let samples: Vec<TouchSample> = points
            .iter()
            .map(|[across, down, millis]| {
                TouchSample::new(
                    PlanePoint::new(Pixels::new(*across), Pixels::new(*down)),
                    start + Duration::from_secs_f64(millis / 1000.0),
                )
            })
            .collect();
        let (first, rest) = samples.split_first().expect("a touch");
        let (last, middle) = rest.split_last().expect("an end");
        let origin =
            held.unwrap_or_else(|| Origin::on_page(first.point()[guide::plane::Axis::Across]));
        middle
            .iter()
            .fold(Gesture::pressed(origin, *first), |gesture, sample| {
                gesture.moved(*sample)
            })
            .outcome(*last, Viewport::default())
    }
}

const ON_PAGE: Option<Origin> = None;
const ON_MENU: Option<Origin> = Some(Origin::Menu);
const SLOW_OPEN: [[f64; 3]; 5] = [
    [10.0, 500.0, 0.0],
    [70.0, 502.0, 200.0],
    [140.0, 504.0, 400.0],
    [215.0, 506.0, 600.0],
    [215.0, 506.0, 800.0],
];
const FLICK_OPEN: [[f64; 3]; 3] = [
    [22.0, 500.0, 0.0],
    [52.0, 502.0, 16.0],
    [112.0, 503.0, 32.0],
];
const SHORT_PEEK: [[f64; 3]; 4] = [
    [10.0, 500.0, 0.0],
    [40.0, 502.0, 16.0],
    [60.0, 503.0, 32.0],
    [60.0, 503.0, 300.0],
];
const FLICK_CLOSE: [[f64; 3]; 3] = [
    [300.0, 400.0, 0.0],
    [270.0, 402.0, 16.0],
    [210.0, 404.0, 32.0],
];
const DOWN_THE_EDGE: [[f64; 3]; 3] = [[10.0, 300.0, 0.0], [20.0, 360.0, 16.0], [40.0, 460.0, 32.0]];
const TAP_AT_THE_EDGE: [[f64; 3]; 2] = [[10.0, 300.0, 0.0], [12.0, 301.0, 90.0]];
const MID_SCREEN: [[f64; 3]; 3] = [
    [60.0, 400.0, 0.0],
    [120.0, 402.0, 16.0],
    [260.0, 404.0, 32.0],
];

#[test]
fn a_long_slow_swipe_or_a_quick_flick_from_the_edge_opens_the_menu() {
    assert_eq!(
        [
            Swipe::ending(ON_PAGE, &SLOW_OPEN),
            Swipe::ending(ON_PAGE, &FLICK_OPEN)
        ],
        [Outcome::OpenMenu, Outcome::OpenMenu]
    );
}

#[test]
fn a_short_peek_snaps_back_and_a_flick_left_closes() {
    assert_eq!(
        [
            Swipe::ending(ON_PAGE, &SHORT_PEEK),
            Swipe::ending(ON_MENU, &FLICK_CLOSE)
        ],
        [Outcome::CloseMenu, Outcome::CloseMenu]
    );
}

#[test]
fn taps_and_vertical_drags_do_nothing_and_swipes_away_from_the_edge_turn_the_page() {
    assert_eq!(
        [&DOWN_THE_EDGE[..], &TAP_AT_THE_EDGE, &MID_SCREEN]
            .map(|points| Swipe::ending(ON_PAGE, points)),
        [
            Outcome::Nothing,
            Outcome::Nothing,
            Outcome::Swipe(Toward::Previous)
        ]
    );
}
