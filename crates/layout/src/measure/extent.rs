use guide::map_frame::Pixels;
use guide::words::Label;

use crate::scale::Token;
use crate::scale::Viewport;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extent {
    Glyph,
    Column,
    ScaleBar,
    ScaleText,
    MapShortest,
    MapTallest,
}

impl Token for Extent {
    const ALL: &'static [Extent] = &[
        Extent::Glyph,
        Extent::Column,
        Extent::ScaleBar,
        Extent::ScaleText,
        Extent::MapShortest,
        Extent::MapTallest,
    ];

    fn name(self) -> Label {
        Label::new(match self {
            Extent::Glyph => "extent-glyph",
            Extent::Column => "extent-column",
            Extent::ScaleBar => "extent-scale-bar",
            Extent::ScaleText => "extent-scale-text",
            Extent::MapShortest => "extent-map-shortest",
            Extent::MapTallest => "extent-map-tallest",
        })
    }

    fn length(self, viewport: Viewport) -> Pixels {
        viewport.scaled(match self {
            Extent::Glyph => Self::GLYPH,
            Extent::Column => Self::COLUMN,
            Extent::ScaleBar => Self::SCALE_BAR,
            Extent::ScaleText => Self::SCALE_TEXT,
            Extent::MapShortest => Self::MAP_SHORTEST,
            Extent::MapTallest => Self::MAP_TALLEST,
        })
    }
}

impl Extent {
    const GLYPH: Pixels = Pixels::new(14.0);
    const COLUMN: Pixels = Pixels::new(16.0);
    const SCALE_BAR: Pixels = Pixels::new(90.0);
    const SCALE_TEXT: Pixels = Pixels::new(60.0);
    const MAP_SHORTEST: Pixels = Pixels::new(190.0);
    const MAP_TALLEST: Pixels = Pixels::new(262.0);
}
