use guide::plane::Pixels;
use guide::plane::Scale;
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

    fn weight(self) -> Scale {
        match self {
            Line::Hair => Self::HAIR,
            Line::Firm => Self::FIRM,
            Line::Bold => Self::BOLD,
            Line::Heavy => Self::HEAVY,
        }
    }
}

impl Token for Line {
    const ALL: &'static [Line] = &[Line::Hair, Line::Firm, Line::Bold, Line::Heavy];

    fn name(self) -> Label {
        match self {
            Line::Hair => Label::new("line-hair"),
            Line::Firm => Label::new("line-firm"),
            Line::Bold => Label::new("line-bold"),
            Line::Heavy => Label::new("line-heavy"),
        }
    }

    fn length(self, viewport: Viewport) -> Pixels {
        viewport.scaled(Self::HAIRLINE * self.weight().factor())
    }
}
