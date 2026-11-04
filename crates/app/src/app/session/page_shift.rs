use std::fmt;
use std::time::Duration;

use guide::plane::Pixels;
use layout::Motion;
use layout::scale::Viewport;

use crate::app::session::hand::Swipe;
use crate::app::step::Toward;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageShift {
    toward: Option<Toward>,
    nudge: Pixels,
}

impl PageShift {
    #[must_use]
    pub fn new(toward: Option<Toward>, viewport: Viewport) -> PageShift {
        PageShift {
            toward,
            nudge: Swipe::Nudge.length(viewport),
        }
    }
}

struct Follow(Duration);

impl fmt::Display for Follow {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}ms", self.0.as_millis())
    }
}

impl fmt::Display for PageShift {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.toward {
            None => write!(
                formatter,
                "--shift: none; --follow: {}",
                Follow(Duration::from(Motion::Slide))
            ),
            Some(Toward::Next) => write!(
                formatter,
                "--shift: translateX({}px); --follow: {}",
                -self.nudge,
                Follow(Duration::from(Motion::Slide))
            ),
            Some(Toward::Previous) => write!(
                formatter,
                "--shift: translateX({}px); --follow: {}",
                self.nudge,
                Follow(Duration::from(Motion::Slide))
            ),
        }
    }
}
