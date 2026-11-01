use guide::Trip;
use map_class::Zoom;
use tile_shapes::Corridor;
use tiles::TileId;

#[test]
fn a_tile_beyond_the_corridor_is_not_held() {
    let corridor = Corridor::of_trip(
        &include_str!("../../../../data/trips/sample.toml")
            .parse::<Trip>()
            .expect("the sample trip loads"),
    );
    assert!(!corridor.holds(TileId::new(Zoom::MOST, 0, 0)));
    assert!(corridor.outline().to_string().contains("MultiPolygon"));
}
