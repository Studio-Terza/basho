use guide::geo::Share;
use guide::map_frame::{Pixels, Scale};

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
    const GUTTERS: Scale = Scale::new(2.0);

    #[must_use]
    pub fn width(self, viewport: Viewport) -> Pixels {
        match self {
            Panel::Menu => viewport.across(Self::MENU),
            Panel::Page => {
                viewport.across(Self::WHOLE) - Gap::Gutter.length(viewport) * Self::GUTTERS.factor()
            }
        }
    }
}
