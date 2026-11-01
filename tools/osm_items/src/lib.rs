mod osm_tags;
mod readers;

use std::str::FromStr;

use serde::Deserialize;
use tile_draft::TileItem;
use tile_shapes::{LonLat, TileShape, WorldPoint};

use osm_tags::OsmTags;
use readers::{OtherLine, PlaceSpot, PoiSpot, RoadLine, SolidArea, TagReader, WaterLine};

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", content = "coordinates")]
enum OsmGeometry {
    Point(LonLat),
    LineString(Vec<LonLat>),
    MultiPolygon(Vec<Vec<Vec<LonLat>>>),
}

#[derive(Clone, Debug, Deserialize)]
pub struct OsmLine {
    geometry: OsmGeometry,
    properties: OsmTags,
}

impl FromStr for OsmLine {
    type Err = serde_json::Error;

    fn from_str(line: &str) -> Result<OsmLine, serde_json::Error> {
        serde_json::from_str(line)
    }
}

impl OsmLine {
    #[must_use]
    pub fn tile_item(&self) -> Option<TileItem> {
        let world = |path: &Vec<LonLat>| -> Vec<WorldPoint> {
            path.iter().copied().map(WorldPoint::of_lon_lat).collect()
        };
        match &self.geometry {
            OsmGeometry::Point(point) => self.spot_item(WorldPoint::of_lon_lat(*point)),
            OsmGeometry::LineString(line) => self.line_item(world(line)),
            OsmGeometry::MultiPolygon(polygons) => self.area_item(
                polygons
                    .iter()
                    .map(|rings| rings.iter().map(world).collect())
                    .collect(),
            ),
        }
    }

    fn line_item(&self, line: Vec<WorldPoint>) -> Option<TileItem> {
        let tags = &self.properties;
        RoadLine::drawn_as(tags)
            .or_else(|| WaterLine::drawn_as(tags))
            .or_else(|| OtherLine::drawn_as(tags))
            .map(|drawn| TileItem::new(drawn, TileShape::Line(line)))
    }

    fn spot_item(&self, point: WorldPoint) -> Option<TileItem> {
        let tags = &self.properties;
        PlaceSpot::drawn_as(tags)
            .or_else(|| PoiSpot::drawn_as(tags))
            .map(|drawn| TileItem::new(drawn, TileShape::Spot(point)))
    }

    fn area_item(&self, polygons: Vec<Vec<Vec<WorldPoint>>>) -> Option<TileItem> {
        let middle = polygons
            .first()
            .and_then(|rings| rings.first())
            .and_then(|ring| WorldPoint::middle_of(ring));
        match SolidArea::drawn_as(&self.properties) {
            Some(drawn) => Some(TileItem::new(drawn, TileShape::Area(polygons))),
            None => middle.and_then(|point| self.spot_item(point)),
        }
    }
}
