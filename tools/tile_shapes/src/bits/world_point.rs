use guide::geo::Coordinate;
use std::f64::consts::PI;
use std::ops::{Add, Sub};

use map_class::{GridAxis, Zoom};
use tiles::TileId;

use super::LonLat;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldPoint {
    across: f64,
    down: f64,
}

impl WorldPoint {
    pub const EXTENT: f64 = 4096.0;
    pub const ORIGIN: WorldPoint = WorldPoint {
        across: 0.0,
        down: 0.0,
    };

    #[must_use]
    pub fn of_lon_lat(place: LonLat) -> WorldPoint {
        let turned = place.degrees_along(GridAxis::Down).to_radians();
        WorldPoint {
            across: (place.degrees_along(GridAxis::Across) + 180.0) / 360.0,
            down: (1.0 - (turned.tan() + 1.0 / turned.cos()).ln() / PI) / 2.0,
        }
    }

    #[must_use]
    pub fn of_coordinate(point: Coordinate) -> WorldPoint {
        Self::of_lon_lat(LonLat::new(
            point.lon().angle().degrees(),
            point.lat().angle().degrees(),
        ))
    }

    #[must_use]
    pub fn of_tile_corner(tile: TileId) -> WorldPoint {
        let across = f64::from(tile.zoom().tiles_across());
        WorldPoint {
            across: f64::from(tile.index_along(GridAxis::Across)) / across,
            down: f64::from(tile.index_along(GridAxis::Down)) / across,
        }
    }

    #[must_use]
    pub fn along(self, axis: GridAxis) -> f64 {
        match axis {
            GridAxis::Across => self.across,
            GridAxis::Down => self.down,
        }
    }

    #[must_use]
    pub fn lon_lat(self) -> LonLat {
        LonLat::new(
            self.across * 360.0 - 180.0,
            (PI * (1.0 - 2.0 * self.down)).sinh().atan().to_degrees(),
        )
    }

    #[must_use]
    pub fn toward(self, other: WorldPoint, share: f64) -> WorldPoint {
        WorldPoint {
            across: self.across + (other.across - self.across) * share,
            down: self.down + (other.down - self.down) * share,
        }
    }

    #[must_use]
    pub fn in_tile_units(self, zoom: Zoom) -> WorldPoint {
        let scale = f64::from(zoom.tiles_across()) * Self::EXTENT;
        WorldPoint {
            across: self.across * scale,
            down: self.down * scale,
        }
    }

    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a tile index on the world grid is a small non-negative number"
    )]
    #[must_use]
    pub const fn tile_index(units: f64) -> u32 {
        (units / Self::EXTENT).floor() as u32
    }

    #[must_use]
    pub fn inside_tile(self) -> Option<WorldPoint> {
        [self.across, self.down]
            .iter()
            .all(|along| (0.0..Self::EXTENT).contains(along))
            .then_some(self)
    }

    #[must_use]
    pub fn reach_of(ring: &[WorldPoint]) -> f64 {
        [GridAxis::Across, GridAxis::Down]
            .into_iter()
            .map(|axis| Self::span_along(ring, axis))
            .sum()
    }

    fn span_along(ring: &[WorldPoint], axis: GridAxis) -> f64 {
        let along = || ring.iter().map(|point| point.along(axis));
        along().fold(f64::NEG_INFINITY, f64::max) - along().fold(f64::INFINITY, f64::min)
    }

    #[must_use]
    pub fn middle_of(ring: &[WorldPoint]) -> Option<WorldPoint> {
        let count = f64::from(u32::try_from(ring.len()).ok().filter(|count| *count > 0)?);
        let sum = ring.iter().fold(Self::ORIGIN, |sum, point| sum + *point);
        Some(WorldPoint {
            across: sum.across / count,
            down: sum.down / count,
        })
    }

    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a share of the world times the tiles across is a small tile index"
    )]
    #[must_use]
    pub const fn cell_at(self, zoom: Zoom) -> TileId {
        let across = zoom.tiles_across() as f64;
        TileId::new(
            zoom,
            (self.across * across).floor() as u32,
            (self.down * across).floor() as u32,
        )
    }
}

impl Add for WorldPoint {
    type Output = WorldPoint;

    fn add(self, step: WorldPoint) -> WorldPoint {
        WorldPoint {
            across: self.across + step.across,
            down: self.down + step.down,
        }
    }
}

impl Sub for WorldPoint {
    type Output = WorldPoint;

    fn sub(self, origin: WorldPoint) -> WorldPoint {
        WorldPoint {
            across: self.across - origin.across,
            down: self.down - origin.down,
        }
    }
}
