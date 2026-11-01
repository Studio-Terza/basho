mod origin;
mod outcome;
pub mod stroke;
mod touch_sample;

use guide::map_frame::Pixels;

use guide::geo::Share;
use layout::scale::Viewport;
use layout::{Panel, Touch};
use menu::{DragFrom, Pull};

use crate::app::session::hand::Flick;
use crate::app::step::Toward;
pub use origin::Origin;
pub use outcome::Outcome;
use stroke::Stroke;
pub use touch_sample::TouchSample;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Gesture {
    #[default]
    Idle,
    Pressed {
        origin: Origin,
        start: TouchSample,
    },
    Dragging {
        origin: Origin,
        start: TouchSample,
        latest: TouchSample,
        flick: Flick,
    },
    Paired {
        start: TouchSample,
        spread: Pixels,
    },
    Scrolling,
}

impl Gesture {
    const HALFWAY: Share = Share::new(0.5);

    #[must_use]
    pub fn pressed(origin: Origin, start: TouchSample) -> Gesture {
        Gesture::Pressed { origin, start }
    }

    #[must_use]
    pub fn paired(start: TouchSample, spread: Pixels) -> Gesture {
        Gesture::Paired { start, spread }
    }

    #[must_use]
    pub fn spread_to(self, now: Pixels) -> Gesture {
        match self {
            Gesture::Paired { spread, .. } if (now - spread).abs() >= Touch::Slop.extent() => {
                Gesture::Scrolling
            }
            other => other,
        }
    }

    #[must_use]
    pub fn moved(self, sample: TouchSample) -> Gesture {
        match self {
            Gesture::Paired { start, .. } => {
                Stroke::between(start, sample).travel().pair_after(self)
            }
            Gesture::Pressed { origin, start } => Stroke::between(start, sample)
                .travel()
                .gesture_after(self, origin, start, sample),
            Gesture::Dragging {
                origin,
                start,
                latest,
                flick,
            } => Gesture::Dragging {
                origin,
                start,
                latest: sample,
                flick: flick.followed(
                    sample.point()[guide::map_frame::Axis::Across]
                        - latest.point()[guide::map_frame::Axis::Across],
                    latest.time(),
                    sample.time(),
                ),
            },
            Gesture::Idle | Gesture::Scrolling => self,
        }
    }

    #[must_use]
    pub fn origin(self) -> Option<Origin> {
        match self {
            Gesture::Pressed { origin, .. } | Gesture::Dragging { origin, .. } => Some(origin),
            Gesture::Idle | Gesture::Paired { .. } | Gesture::Scrolling => None,
        }
    }

    #[must_use]
    pub fn swipe(self, viewport: Viewport) -> Option<Toward> {
        match self {
            Gesture::Dragging {
                origin: Origin::Page,
                start,
                latest,
                ..
            } => Stroke::between(start, latest).swipe(viewport),
            Gesture::Dragging { .. }
            | Gesture::Pressed { .. }
            | Gesture::Idle
            | Gesture::Paired { .. }
            | Gesture::Scrolling => None,
        }
    }

    #[must_use]
    pub fn pull(self, viewport: Viewport) -> Option<Pull> {
        match self {
            Gesture::Dragging {
                origin: Origin::Edge,
                start,
                latest,
                ..
            } => Some(Pull::dragged(
                DragFrom::Edge,
                start.point(),
                latest.point(),
                viewport,
            )),
            Gesture::Dragging {
                origin: Origin::Menu,
                start,
                latest,
                ..
            } => Some(Pull::dragged(
                DragFrom::Menu,
                start.point(),
                latest.point(),
                viewport,
            )),
            Gesture::Dragging {
                origin: Origin::Page,
                ..
            }
            | Gesture::Pressed { .. }
            | Gesture::Idle
            | Gesture::Paired { .. }
            | Gesture::Scrolling => None,
        }
    }

    #[must_use]
    pub fn abandoned(self) -> Outcome {
        match self {
            Gesture::Dragging {
                origin: Origin::Edge,
                ..
            } => Outcome::CloseMenu,
            Gesture::Dragging { .. }
            | Gesture::Pressed { .. }
            | Gesture::Idle
            | Gesture::Paired { .. }
            | Gesture::Scrolling => Outcome::Nothing,
        }
    }

    pub fn outcome(self, end: TouchSample, viewport: Viewport) -> Outcome {
        let finished = self.moved(end);
        let width = Panel::Menu.width(viewport);
        match finished {
            Gesture::Dragging {
                origin: Origin::Edge,
                flick,
                ..
            } => {
                let pull = finished.pull(viewport).map_or(Pixels::ZERO, Pull::reach);
                if flick > Flick::COMMIT
                    || (pull > width * Self::HALFWAY.fraction() && flick > Flick::COMMIT.reversed())
                {
                    Outcome::OpenMenu
                } else {
                    Outcome::CloseMenu
                }
            }
            Gesture::Dragging {
                origin: Origin::Menu,
                flick,
                ..
            } => {
                let pull = finished.pull(viewport).map_or(width, Pull::reach);
                if flick < Flick::COMMIT.reversed()
                    || (pull < width * Self::HALFWAY.fraction() && flick < Flick::COMMIT)
                {
                    Outcome::CloseMenu
                } else {
                    Outcome::KeepMenu
                }
            }
            Gesture::Dragging {
                origin: Origin::Page,
                ..
            } => finished
                .swipe(viewport)
                .map_or(Outcome::Nothing, Outcome::Swipe),
            Gesture::Pressed { .. }
            | Gesture::Idle
            | Gesture::Paired { .. }
            | Gesture::Scrolling => Outcome::Nothing,
        }
    }
}
