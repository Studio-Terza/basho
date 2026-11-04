use guide::plane::Pixels;
use guide::plane::Scale;
use guide::words::Label;

use crate::scale::token::Token;
use crate::scale::viewport::Viewport;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gap {
    Hair,
    Thin,
    Tight,
    Snug,
    Close,
    Easy,
    Roomy,
    Gutter,
    Loose,
    Open,
}

impl Gap {
    const STEP: Pixels = Pixels::new(2.0);
    const HAIR: Scale = Scale::new(0.5);
    const THIN: Scale = Scale::new(1.0);
    const TIGHT: Scale = Scale::new(2.0);
    const SNUG: Scale = Scale::new(3.0);
    const CLOSE: Scale = Scale::new(4.0);
    const EASY: Scale = Scale::new(5.0);
    const ROOMY: Scale = Scale::new(6.0);
    const GUTTER: Scale = Scale::new(8.0);
    const LOOSE: Scale = Scale::new(12.0);
    const OPEN: Scale = Scale::new(16.0);
}

impl Token for Gap {
    const ALL: &'static [Gap] = &[
        Gap::Hair,
        Gap::Thin,
        Gap::Tight,
        Gap::Snug,
        Gap::Close,
        Gap::Easy,
        Gap::Roomy,
        Gap::Gutter,
        Gap::Loose,
        Gap::Open,
    ];

    fn name(self) -> Label {
        match self {
            Gap::Hair => Label::new("gap-hair"),
            Gap::Thin => Label::new("gap-thin"),
            Gap::Tight => Label::new("gap-tight"),
            Gap::Snug => Label::new("gap-snug"),
            Gap::Close => Label::new("gap-close"),
            Gap::Easy => Label::new("gap-easy"),
            Gap::Roomy => Label::new("gap-roomy"),
            Gap::Gutter => Label::new("gap-gutter"),
            Gap::Loose => Label::new("gap-loose"),
            Gap::Open => Label::new("gap-open"),
        }
    }

    fn length(self, viewport: Viewport) -> Pixels {
        viewport.scaled(Self::STEP * self.steps().factor())
    }
}

impl Gap {
    fn steps(self) -> Scale {
        match self {
            Gap::Hair => Self::HAIR,
            Gap::Thin => Self::THIN,
            Gap::Tight => Self::TIGHT,
            Gap::Snug => Self::SNUG,
            Gap::Close => Self::CLOSE,
            Gap::Easy => Self::EASY,
            Gap::Roomy => Self::ROOMY,
            Gap::Gutter => Self::GUTTER,
            Gap::Loose => Self::LOOSE,
            Gap::Open => Self::OPEN,
        }
    }
}
