use std::ops::RangeInclusive;

use map_class::{GridAxis, Zoom};
use tiles::TileId;

use super::{ClipSide, GridPoint, LineRuns, PathKind, ShapeCut, Thinning, WorldPoint};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Winding {
    Outer,
    Hole,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileShape {
    Spot(WorldPoint),
    Line(Vec<WorldPoint>),
    Area(Vec<Vec<Vec<WorldPoint>>>),
}

impl TileShape {
    const LEAST_AREA_REACH: f64 = 256.0;
    const BUFFER: f64 = 64.0;

    #[must_use]
    pub fn drawn_at(&self, zoom: Zoom) -> Option<TileShape> {
        match self {
            TileShape::Spot(point) => Some(TileShape::Spot(point.in_tile_units(zoom))),
            TileShape::Line(line) => Some(TileShape::Line(Thinning::drawn_at(line, zoom))),
            TileShape::Area(polygons) => {
                let drawn: Vec<Vec<Vec<WorldPoint>>> = polygons
                    .iter()
                    .filter_map(|rings| Self::polygon_at(zoom, rings))
                    .collect();
                (!drawn.is_empty()).then_some(TileShape::Area(drawn))
            }
        }
    }

    fn polygon_at(zoom: Zoom, rings: &[Vec<WorldPoint>]) -> Option<Vec<Vec<WorldPoint>>> {
        let drawn: Vec<Vec<WorldPoint>> = rings
            .iter()
            .map(|ring| Thinning::drawn_at(ring, zoom))
            .collect();
        (zoom == Zoom::MOST || WorldPoint::reach_of(drawn.first()?) >= Self::LEAST_AREA_REACH)
            .then_some(drawn)
    }

    #[must_use]
    pub fn tiles_at(&self, zoom: Zoom) -> Vec<TileId> {
        let points: Vec<WorldPoint> = match self {
            TileShape::Spot(point) => vec![*point],
            TileShape::Line(line) => line.clone(),
            TileShape::Area(polygons) => polygons.iter().flatten().flatten().copied().collect(),
        };
        let rows = Self::span_along(&points, GridAxis::Down);
        Self::span_along(&points, GridAxis::Across)
            .flat_map(|column| rows.clone().map(move |row| TileId::new(zoom, column, row)))
            .collect()
    }

    #[must_use]
    pub fn cut_in(&self, tile: TileId) -> Option<ShapeCut> {
        let corner = WorldPoint::of_tile_corner(tile).in_tile_units(tile.zoom());
        let local = |path: &[WorldPoint]| -> Vec<WorldPoint> {
            path.iter().map(|point| *point - corner).collect()
        };
        match self {
            TileShape::Spot(point) => (*point - corner)
                .inside_tile()
                .and_then(|at| ShapeCut::new(PathKind::Spot, vec![vec![GridPoint::from(at)]])),
            TileShape::Line(line) => {
                let runs = LineRuns::inside(&local(line))
                    .iter()
                    .map(|run| GridPoint::path_from(run))
                    .filter(|run| run.len() > 1)
                    .collect();
                ShapeCut::new(PathKind::Line, runs)
            }
            TileShape::Area(polygons) => {
                let kept = polygons
                    .iter()
                    .flat_map(|rings| Self::polygon_in(rings, corner))
                    .collect();
                ShapeCut::new(PathKind::Area, kept)
            }
        }
    }

    fn polygon_in(rings: &[Vec<WorldPoint>], corner: WorldPoint) -> Vec<Vec<GridPoint>> {
        let local = |ring: &Vec<WorldPoint>| -> Vec<WorldPoint> {
            ring.iter().map(|point| *point - corner).collect()
        };
        let outer = rings
            .first()
            .and_then(|ring| Self::ring_in(&local(ring), Winding::Outer));
        let holes = rings
            .iter()
            .skip(1)
            .filter(|_| outer.is_some())
            .filter_map(|ring| Self::ring_in(&local(ring), Winding::Hole));
        outer.iter().cloned().chain(holes).collect()
    }

    fn ring_in(ring: &[WorldPoint], winding: Winding) -> Option<Vec<GridPoint>> {
        let open = ring.split_last().map_or(ring, |(_, rest)| rest);
        let clipped = ClipSide::ALL
            .into_iter()
            .fold(open.to_vec(), |kept, side| side.clipped(&kept));
        GridPoint::ring_from(&clipped).map(|grid| {
            if (GridPoint::doubled_area(&grid) > 0) == (winding == Winding::Outer) {
                grid
            } else {
                grid.into_iter().rev().collect()
            }
        })
    }

    fn span_along(points: &[WorldPoint], axis: GridAxis) -> RangeInclusive<u32> {
        let along = || points.iter().map(|point| point.along(axis));
        WorldPoint::tile_index(along().fold(f64::INFINITY, f64::min) - Self::BUFFER)
            ..=WorldPoint::tile_index(along().fold(f64::NEG_INFINITY, f64::max) + Self::BUFFER)
    }
}
