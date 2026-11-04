use guide::plane::Pixels;
use guide::words::Label;

use crate::scale::Gap;
use crate::scale::Token;
use crate::scale::Viewport;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corner {
    Small,
    Round,
}

impl Token for Corner {
    const ALL: &'static [Corner] = &[Corner::Small, Corner::Round];

    fn name(self) -> Label {
        match self {
            Corner::Small => Label::new("corner-small"),
            Corner::Round => Label::new("corner-round"),
        }
    }

    fn length(self, viewport: Viewport) -> Pixels {
        match self {
            Corner::Small => Gap::Tight.length(viewport),
            Corner::Round => Gap::Close.length(viewport),
        }
    }
}
