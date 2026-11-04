use std::marker::PhantomData;

use map_class::{LandKind, PlaceRank, PoiGroup, RoadRank, RoadSurface, SurfaceTag, Zoom};
use tile_draft::TileLayer;
use tile_draft::{DrawnAs, TagKey, TileTag};

use crate::osm_tags::{OsmKey, OsmTags};

pub trait TagReader {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs>;
}

pub struct OrElse<First, Then> {
    first: PhantomData<First>,
    then: PhantomData<Then>,
}

impl<First: TagReader, Then: TagReader> TagReader for OrElse<First, Then> {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs> {
        First::drawn_as(tags).or_else(|| Then::drawn_as(tags))
    }
}

pub type LineReader = OrElse<RoadLine, OrElse<WaterLine, OtherLine>>;
pub type SpotReader = OrElse<PlaceSpot, PoiSpot>;

pub struct RoadLine;

impl TagReader for RoadLine {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs> {
        let rank: RoadRank = tags.parsed(OsmKey::HIGHWAY)?;
        let surface = RoadSurface::of_way(rank, tags.parsed::<SurfaceTag>(OsmKey::SURFACE));
        let kept = [
            TileTag::new(TagKey::Class, <&str>::from(rank).into()),
            TileTag::new(TagKey::Surface, <&str>::from(surface).into()),
        ]
        .into_iter()
        .chain(tags.tag_of(TagKey::Structure))
        .chain(tags.tag_of(TagKey::Ref))
        .chain(tags.label_tags())
        .collect();
        Some(DrawnAs::new(TileLayer::Road, rank.into(), kept))
    }
}

pub struct WaterLine;

impl WaterLine {
    const FIRST_ZOOMS: [(&str, Zoom); 5] = [
        ("river", Zoom::RIVERS),
        ("canal", Zoom::LOCAL_ROADS),
        ("stream", Zoom::STREETS),
        ("drain", Zoom::CELL),
        ("ditch", Zoom::CELL),
    ];
}

impl TagReader for WaterLine {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs> {
        let waterway = tags.text(OsmKey::WATERWAY)?;
        let first_zoom = Self::FIRST_ZOOMS
            .into_iter()
            .find(|(name, _)| *name == waterway)
            .map(|(_, zoom)| zoom)?;
        Some(DrawnAs::new(
            TileLayer::WaterLine,
            first_zoom,
            tags.label_tags(),
        ))
    }
}

pub struct OtherLine;

impl OtherLine {
    const RAIL_ZOOMS: [(&str, Zoom); 7] = [
        ("rail", Zoom::LEAST),
        ("subway", Zoom::LOCAL_ROADS),
        ("light_rail", Zoom::LOCAL_ROADS),
        ("narrow_gauge", Zoom::LOCAL_ROADS),
        ("monorail", Zoom::LOCAL_ROADS),
        ("tram", Zoom::LOCAL_ROADS),
        ("funicular", Zoom::LOCAL_ROADS),
    ];
    const NATURAL_LAYERS: [(&str, TileLayer, Zoom); 4] = [
        ("coastline", TileLayer::Coast, Zoom::LEAST),
        ("cliff", TileLayer::Terrain, Zoom::CELL),
        ("ridge", TileLayer::Terrain, Zoom::CELL),
        ("arete", TileLayer::Terrain, Zoom::CELL),
    ];
}

impl TagReader for OtherLine {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs> {
        let plain = |layer, zoom| Some(DrawnAs::new(layer, zoom, Vec::new()));
        let natural = tags.text(OsmKey::NATURAL);
        tags.text(OsmKey::RAILWAY)
            .and_then(|railway| {
                Self::RAIL_ZOOMS
                    .into_iter()
                    .find(|(name, _)| *name == railway)
                    .map(|(_, zoom)| zoom)
            })
            .map(|zoom| {
                let kept = tags.tag_of(TagKey::Structure).into_iter().collect();
                DrawnAs::new(TileLayer::Rail, zoom, kept)
            })
            .or_else(|| {
                Self::NATURAL_LAYERS
                    .into_iter()
                    .find(|(name, _, _)| Some(*name) == natural)
                    .and_then(|(_, layer, zoom)| plain(layer, zoom))
            })
            .or_else(|| {
                tags.text(OsmKey::ROUTE)
                    .filter(|route| *route == "ferry")
                    .and_then(|_| plain(TileLayer::Ferry, Zoom::LEAST))
            })
            .or_else(|| {
                tags.text(OsmKey::AERIALWAY)
                    .and_then(|_| plain(TileLayer::Terrain, Zoom::STREETS))
            })
    }
}

pub struct PlaceSpot;

impl TagReader for PlaceSpot {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs> {
        let rank: PlaceRank = tags.parsed(OsmKey::PLACE)?;
        let kept = [TileTag::new(TagKey::Class, <&str>::from(rank).into())]
            .into_iter()
            .chain(tags.label_tags())
            .collect();
        Some(DrawnAs::new(TileLayer::Place, rank.into(), kept))
    }
}

pub struct PoiSpot;

impl TagReader for PoiSpot {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs> {
        let group = [
            OsmKey::AMENITY,
            OsmKey::TOURISM,
            OsmKey::RAILWAY,
            OsmKey::NATURAL,
        ]
        .into_iter()
        .find_map(|key| tags.parsed(key))
        .or_else(|| tags.text(OsmKey::HISTORIC).map(|_| PoiGroup::Sight))
        .or_else(|| {
            tags.text(OsmKey::SHOP)
                .map(|shop| shop.parse().unwrap_or(PoiGroup::Shop))
        })?;
        let kept = [TileTag::new(TagKey::Class, <&str>::from(group).into())]
            .into_iter()
            .chain(tags.label_tags())
            .collect();
        Some(DrawnAs::new(TileLayer::Poi, group.into(), kept))
    }
}

pub struct SolidArea;

impl TagReader for SolidArea {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs> {
        let water = tags
            .text(OsmKey::NATURAL)
            .filter(|natural| *natural == "water")
            .or_else(|| tags.text(OsmKey::WATERWAY))
            .or_else(|| {
                tags.text(OsmKey::LANDUSE)
                    .filter(|landuse| *landuse == "reservoir")
            });
        tags.text(OsmKey::BUILDING)
            .map(|_| DrawnAs::new(TileLayer::Building, Zoom::MOST, Vec::new()))
            .or_else(|| water.map(|_| DrawnAs::new(TileLayer::Water, Zoom::LEAST, Vec::new())))
            .or_else(|| {
                [OsmKey::NATURAL, OsmKey::LANDUSE, OsmKey::LEISURE]
                    .into_iter()
                    .find_map(|key| tags.parsed::<LandKind>(key))
                    .map(|kind| {
                        let class = TileTag::new(TagKey::Class, <&str>::from(kind).into());
                        DrawnAs::new(TileLayer::Land, kind.into(), vec![class])
                    })
            })
    }
}
