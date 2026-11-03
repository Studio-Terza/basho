use map_class::{LandKind, PlaceRank, PoiGroup, RoadRank, RoadSurface, SurfaceTag, Zoom};
use tile_draft::TileLayer;
use tile_draft::{DrawnAs, TagKey, TileTag};

use crate::osm_tags::{OsmKey, OsmTags};

pub trait TagReader {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs>;
}

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

impl TagReader for WaterLine {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs> {
        let first_zoom = match tags.text(OsmKey::WATERWAY)? {
            "river" => Some(Zoom::RIVERS),
            "canal" => Some(Zoom::LOCAL_ROADS),
            "stream" => Some(Zoom::STREETS),
            "drain" | "ditch" => Some(Zoom::CELL),
            _ => None,
        }?;
        Some(DrawnAs::new(
            TileLayer::WaterLine,
            first_zoom,
            tags.label_tags(),
        ))
    }
}

pub struct OtherLine;

impl TagReader for OtherLine {
    fn drawn_as(tags: &OsmTags) -> Option<DrawnAs> {
        let rail = |tag| match tag {
            "rail" => Some(Zoom::LEAST),
            "subway" | "light_rail" | "narrow_gauge" | "monorail" | "tram" | "funicular" => {
                Some(Zoom::LOCAL_ROADS)
            }
            _ => None,
        };
        let plain = |layer, zoom| Some(DrawnAs::new(layer, zoom, Vec::new()));
        let natural = tags.text(OsmKey::NATURAL);
        tags.text(OsmKey::RAILWAY)
            .and_then(rail)
            .map(|zoom| {
                let kept = tags.tag_of(TagKey::Structure).into_iter().collect();
                DrawnAs::new(TileLayer::Rail, zoom, kept)
            })
            .or_else(|| match natural {
                Some("coastline") => plain(TileLayer::Coast, Zoom::LEAST),
                Some("cliff" | "ridge" | "arete") => plain(TileLayer::Terrain, Zoom::CELL),
                _ => None,
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
        tags.text(OsmKey::BUILDING)
            .map(|_| DrawnAs::new(TileLayer::Building, Zoom::MOST, Vec::new()))
            .or_else(|| {
                (tags.text(OsmKey::NATURAL) == Some("water")
                    || tags.text(OsmKey::WATERWAY).is_some()
                    || tags.text(OsmKey::LANDUSE) == Some("reservoir"))
                .then(|| DrawnAs::new(TileLayer::Water, Zoom::LEAST, Vec::new()))
            })
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
