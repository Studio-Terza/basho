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
        layer.add(tags, kind as u64, &geometry);
    }
}
