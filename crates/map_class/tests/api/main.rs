use map_class::{PoiGroup, RoadRank, RoadSurface, SurfaceTag, Zoom};

#[test]
fn road_tags_land_in_the_ranks_the_key_names() {
    assert_eq!("trunk_link".parse(), Ok(RoadRank::Highway));
    assert_eq!("footway".parse(), Ok(RoadRank::Trail));
    assert!("proposed".parse::<RoadRank>().is_err());
    assert_eq!(<&str>::from(RoadRank::Highway), "highway");
}

#[test]
fn a_surface_defaults_by_rank_and_a_tag_wins() {
    let surface = RoadSurface::of_way;
    assert_eq!(surface(RoadRank::Track, None), RoadSurface::Unpaved);
    assert_eq!(surface(RoadRank::Trail, None), RoadSurface::Trail);
    assert_eq!(
        surface(RoadRank::Street, Some(SurfaceTag::Loose)),
        RoadSurface::Unpaved
    );
    assert_eq!(
        surface(RoadRank::Trail, Some(SurfaceTag::Hard)),
        RoadSurface::Paved
    );
}

#[test]
fn a_konbini_is_told_apart_from_other_shops_and_shows_late() {
    assert_eq!("convenience".parse(), Ok(PoiGroup::Konbini));
    assert_eq!("restaurant".parse(), Ok(PoiGroup::Food));
    assert_eq!(Zoom::from(PoiGroup::Konbini), Zoom::MOST);
}
