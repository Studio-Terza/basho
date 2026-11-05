use tile_shapes::{PathKind, ShapeCut};
use tiles::TileId;

use super::{GeometryBytes, LayerBytes};
use crate::tile_tag::TileTag;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TilePiece {
    tile: TileId,
    cut: ShapeCut,
}

impl TilePiece {
    const SPOT_CODE: u64 = 1;
    const LINE_CODE: u64 = 2;
    const AREA_CODE: u64 = 3;

    #[must_use]
    pub fn new(tile: TileId, cut: ShapeCut) -> TilePiece {
        TilePiece { tile, cut }
    }

    #[must_use]
    pub fn tile(&self) -> TileId {
        self.tile
    }

    pub fn add_to(&self, layer: &mut LayerBytes, tags: &[TileTag]) {
        let kind = self.cut.kind();
        let mut geometry = GeometryBytes::default();
        self.cut.paths().iter().for_each(|path| {
            geometry.path(path);
            geometry.close_for(kind == PathKind::Area);
        });
        layer.add(tags, Self::geometry_code(kind), &geometry);
    }

    fn geometry_code(kind: PathKind) -> u64 {
        match kind {
            PathKind::Spot => Self::SPOT_CODE,
            PathKind::Line => Self::LINE_CODE,
            PathKind::Area => Self::AREA_CODE,
        }
    }
}
