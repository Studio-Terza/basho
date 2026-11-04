mod boundary;

use std::collections::BTreeMap;
use std::io;

use tile_shapes::{Corridor, TileShape};

use crate::TileLayer;
use map_class::Zoom;
use tiles::{TileId, WrittenArchive};

use crate::tile_item::TileItem;
use boundary::{LayerBytes, TilePiece};

pub struct Pyramid {
    corridor: Corridor,
    tiles: BTreeMap<TileId, BTreeMap<TileLayer, LayerBytes>>,
}

impl Pyramid {
    #[must_use]
    pub fn new(corridor: Corridor) -> Pyramid {
        Pyramid {
            corridor,
            tiles: BTreeMap::new(),
        }
    }

    pub fn take(&mut self, item: &TileItem) {
        let pieces: Vec<TilePiece> = item
            .drawn()
            .first_zoom()
            .up_to_most()
            .flat_map(|zoom| self.pieces_at(item, zoom))
            .collect();
        self.added(item, &pieces);
    }

    fn pieces_at(&self, item: &TileItem, zoom: Zoom) -> Vec<TilePiece> {
        item.shape()
            .drawn_at(zoom)
            .map(|drawn| self.pieces_of(&drawn, zoom))
            .unwrap_or_default()
    }

    fn pieces_of(&self, drawn: &TileShape, zoom: Zoom) -> Vec<TilePiece> {
        drawn
            .tiles_at(zoom)
            .into_iter()
            .filter(|tile| self.corridor.holds(*tile))
            .filter_map(|tile| drawn.cut_in(tile).map(|cut| TilePiece::new(tile, cut)))
            .collect()
    }

    #[expect(
        clippy::needless_for_each,
        reason = "expression-flow drives iterators with combinators, never a for loop"
    )]
    fn added(&mut self, item: &TileItem, pieces: &[TilePiece]) {
        pieces.iter().for_each(|piece| {
            let layer = self.tiles.entry(piece.tile()).or_default();
            piece.add_to(
                layer.entry(item.drawn().layer()).or_default(),
                item.drawn().tags(),
            );
        });
    }

    pub fn archive(&self) -> io::Result<WrittenArchive> {
        self.tiles
            .iter()
            .try_fold(WrittenArchive::default(), |mut archive, (tile, layers)| {
                let plain: Vec<u8> = layers
                    .iter()
                    .flat_map(|(layer, bytes)| bytes.tile_message(*layer))
                    .collect();
                archive.add(*tile, &plain)?;
                Ok(archive)
            })
    }

    #[must_use]
    pub fn zooms(&self) -> Vec<Zoom> {
        let mut zooms: Vec<Zoom> = self.tiles.keys().map(|tile| tile.zoom()).collect();
        zooms.dedup();
        zooms
    }
}
