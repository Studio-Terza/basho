use guide::Scale;
use guide::geo::Coordinate;
use guide::geo::Degrees;
use guide::geo::Share;
use guide::map_frame::Axis;
use guide::map_frame::MapFrame;
use guide::map_frame::Pixels;

const WIDTH: Pixels = Pixels::new(400.0);
const SOUTH_WEST: Coordinate = Coordinate::new(Degrees::new(38.0), Degrees::new(140.0));
const NORTH_EAST: Coordinate = Coordinate::new(Degrees::new(38.02), Degrees::new(140.02));
const ZOOM: Scale = Scale::new(1.5);
const CLOSE: f64 = 1.0;
const ACROSS: Axis = Axis::Across;
const DOWN: Axis = Axis::Down;

#[test]
fn every_framed_point_lands_inside_the_frame_with_north_up() {
    let frame = MapFrame::around(&[SOUTH_WEST, NORTH_EAST], WIDTH).expect("frame");
    let low = frame.project(SOUTH_WEST);
    let high = frame.project(NORTH_EAST);
    assert!(
        low[ACROSS] > Pixels::ZERO
            && high[ACROSS] < WIDTH
            && high[DOWN] > Pixels::ZERO
            && low[DOWN] < frame.extent(DOWN)
    );
    assert!(high[DOWN] < low[DOWN] && high[ACROSS] > low[ACROSS]);
}

#[test]
fn zooming_out_keeps_the_size_and_the_centre() {
    let frame = MapFrame::around(&[SOUTH_WEST, NORTH_EAST], WIDTH).expect("frame");
    let wider = frame.zoomed_out(ZOOM);
    let middle = SOUTH_WEST.toward(NORTH_EAST, Share::HALF);
    assert!((wider.extent(DOWN) - frame.extent(DOWN)).abs().pixels() < CLOSE);
    assert!((wider.span() - frame.span() * ZOOM.factor()).abs().meters() < CLOSE);
    assert!(
        (wider.project(middle)[DOWN] - frame.project(middle)[DOWN])
            .abs()
            .pixels()
            < CLOSE
    );
}
