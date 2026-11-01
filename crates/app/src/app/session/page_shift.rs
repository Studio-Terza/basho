use std::fmt;
use std::time::Duration;

use guide::map_frame::Pixels;
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

impl fmt::Display for PageShift {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let follow = Duration::from(Motion::Slide).as_millis();
        match self.toward {
            None => write!(formatter, "--shift: none; --follow: {follow}ms"),
            Some(Toward::Next) => write!(
                formatter,
                "--shift: translateX({}px); --follow: {follow}ms",
                -self.nudge
            ),
            Some(Toward::Previous) => write!(
                formatter,
                "--shift: translateX({}px); --follow: {follow}ms",
                self.nudge
            ),
        }
    }
}
