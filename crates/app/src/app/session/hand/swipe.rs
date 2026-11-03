use guide::geo::{Degrees, Share};
use guide::plane::Pixels;
use layout::Panel;
use layout::Touch;
use layout::scale::{Gap, Token, Viewport};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swipe {
    Travel,
    Nudge,
}

impl Swipe {
    const STEEPEST: Degrees = Degrees::new(30.0);
    const PAGE_SHARE: Share = Share::new(0.2);

    pub fn length(self, viewport: Viewport) -> Pixels {
        match self {
            Swipe::Travel => Touch::Tap
                .extent()
                .max(Panel::Page.width(viewport) * Self::PAGE_SHARE.fraction()),
            Swipe::Nudge => Gap::Roomy.length(viewport),
        }
    }

    pub fn steepest() -> Share {
        let steepest = Self::STEEPEST.degrees().to_radians();
        Share::ratio(steepest.sin(), steepest.cos())
    }
}
