use guide::Trip;
use map_class::Zoom;
use std::sync::LazyLock;
use tile_draft::{DrawnAs, Pyramid, TileItem};
use tile_shapes::{Corridor, LonLat, TileShape, WorldPoint};

static SAMPLE: LazyLock<Trip> = LazyLock::new(|| {
    include_str!("../../../../data/trips/sample.toml")
        .parse()
        .expect("the sample trip loads")
});

#[test]
fn a_road_in_the_corridor_reaches_every_zoom_from_its_first() {
    let first = SAMPLE.itinerary().days()[0]
        .legs()
        .iter()
        .find_map(|leg| leg.line().points().first().copied())
        .expect("the sample trip has a line");
    let lon = first.lon().angle().degrees();
    let lat = first.lat().angle().degrees();
    let line = [LonLat::new(lon, lat), LonLat::new(lon + 0.001, lat + 0.001)];
    let road = TileItem::new(
        DrawnAs::new(tile_draft::TileLayer::Road, Zoom::LEAST, Vec::new()),
        TileShape::Line(line.into_iter().map(WorldPoint::of_lon_lat).collect()),
    );
    let mut pyramid = Pyramid::new(Corridor::of_trip(&SAMPLE));
    pyramid.take(&road);
    assert_eq!(pyramid.zooms().first(), Some(&Zoom::LEAST));
    assert_eq!(pyramid.zooms().last(), Some(&Zoom::MOST));
    assert!(pyramid.archive().expect("packs").tiles_len() >= 8);
}
