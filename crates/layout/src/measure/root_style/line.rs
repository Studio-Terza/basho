use guide::map_frame::{Pixels, Scale};
use guide::words::Label;

use crate::scale::Token;
use crate::scale::Viewport;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line {
    Hair,
    Firm,
    Bold,
    Heavy,
}

impl Line {
    const HAIRLINE: Pixels = Pixels::new(1.0);
    const HAIR: Scale = Scale::new(1.0);
    const FIRM: Scale = Scale::new(1.5);
    const BOLD: Scale = Scale::new(2.0);
    const HEAVY: Scale = Scale::new(3.0);
}

impl Token for Line {
    const ALL: &'static [Line] = &[Line::Hair, Line::Firm, Line::Bold, Line::Heavy];

    fn name(self) -> Label {
        Label::new(match self {
            Line::Hair => "line-hair",
            Line::Firm => "line-firm",
            Line::Bold => "line-bold",
            Line::Heavy => "line-heavy",
        })
    }

    fn length(self, viewport: Viewport) -> Pixels {
        viewport.scaled(
            Self::HAIRLINE
                * match self {
                    Line::Hair => Self::HAIR,
                    Line::Firm => Self::FIRM,
                    Line::Bold => Self::BOLD,
                    Line::Heavy => Self::HEAVY,
                }
                .factor(),
        )
    }
}
