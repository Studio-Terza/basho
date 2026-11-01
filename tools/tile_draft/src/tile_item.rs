use tile_shapes::TileShape;

use crate::drawn_as::DrawnAs;

#[derive(Clone, Debug, PartialEq)]
pub struct TileItem {
    drawn: DrawnAs,
    shape: TileShape,
}

impl TileItem {
    #[must_use]
    pub fn new(drawn: DrawnAs, shape: TileShape) -> TileItem {
        TileItem { drawn, shape }
    }

    #[must_use]
    pub fn drawn(&self) -> &DrawnAs {
        &self.drawn
    }

    #[must_use]
    pub fn shape(&self) -> &TileShape {
        &self.shape
    }
}
