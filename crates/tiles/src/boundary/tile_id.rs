use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::boundary::GridAxis;
use map_class::Zoom;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileId {
    zoom: Zoom,
    column: u32,
    row: u32,
}

impl TileId {
    #[must_use]
    pub const fn new(zoom: Zoom, column: u32, row: u32) -> TileId {
        TileId { zoom, column, row }
    }

    #[must_use]
    pub const fn zoom(self) -> Zoom {
        self.zoom
    }

    #[must_use]
    pub const fn index_along(self, axis: GridAxis) -> u32 {
        match axis {
            GridAxis::Across => self.column,
            GridAxis::Down => self.row,
        }
    }

    #[must_use]
    pub fn parent(self) -> Option<TileId> {
        (self.zoom.level() > 0).then(|| TileId {
            zoom: self.zoom.zoomed_out(),
            column: self.column >> 1,
            row: self.row >> 1,
        })
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TileIdError {
    #[error("a tile is named zoom/column/row")]
    Shape,
    #[error(transparent)]
    Number(#[from] ParseIntError),
}

impl FromStr for TileId {
    type Err = TileIdError;

    fn from_str(text: &str) -> Result<TileId, TileIdError> {
        match <[&str; TileId::PARTS]>::try_from(text.split('/').collect::<Vec<&str>>()) {
            Ok([zoom, column, row]) => Ok(TileId {
                zoom: Zoom::of_level(zoom.parse()?),
                column: column.parse()?,
                row: row.parse()?,
            }),
            Err(_) => Err(TileIdError::Shape),
        }
    }
}

impl TileId {
    const PARTS: usize = 3;
}
