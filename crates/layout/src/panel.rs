use guide::geo::Share;
use guide::plane::Pixels;
use guide::plane::Scale;

use crate::scale::Gap;
use crate::scale::Token;
use crate::scale::Viewport;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Panel {
    Menu,
    Page,
}

impl Panel {
    const MENU: Share = Share::new(0.86);
    const WHOLE: Share = Share::new(1.0);
    const NO_GUTTERS: Scale = Scale::new(0.0);
    const GUTTERS: Scale = Scale::new(2.0);

    #[must_use]
    pub fn width(self, viewport: Viewport) -> Pixels {
        viewport.across(self.share()) - Gap::Gutter.length(viewport) * self.gutters().factor()
    }

    fn share(self) -> Share {
        match self {
            Panel::Menu => Self::MENU,
            Panel::Page => Self::WHOLE,
        }
    }

    fn gutters(self) -> Scale {
        match self {
            Panel::Menu => Self::NO_GUTTERS,
            Panel::Page => Self::GUTTERS,
        }
    }
}
