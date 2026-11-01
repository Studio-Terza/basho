mod travel;

use guide::map_frame::Pixels;
use layout::Touch;
use layout::scale::Viewport;

use crate::app::session::gesture::touch_sample::TouchSample;
use crate::app::session::hand::Swipe;
use crate::app::step::Toward;
use travel::Travel;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stroke {
    across: Pixels,
    down: Pixels,
}

impl Stroke {
    #[must_use]
    pub fn between(start: TouchSample, end: TouchSample) -> Stroke {
        Stroke {
            across: end.point()[guide::map_frame::Axis::Across]
                - start.point()[guide::map_frame::Axis::Across],
            down: end.point()[guide::map_frame::Axis::Down]
                - start.point()[guide::map_frame::Axis::Down],
        }
    }

    #[must_use]
    pub fn across(self) -> Pixels {
        self.across
    }

    #[must_use]
    pub fn travel(self) -> Travel {
        if self.across.abs().max(self.down.abs()) < Touch::Slop.extent() {
            Travel::Still
        } else if self.down.abs() <= self.across.abs() * Touch::steepest().fraction() {
            Travel::Level(self)
        } else {
            Travel::Steep
        }
    }

    #[must_use]
    pub fn swipe(self, viewport: Viewport) -> Option<Toward> {
        Some(self)
            .filter(|stroke| stroke.across.abs() >= Swipe::Travel.length(viewport))
            .filter(|stroke| {
                stroke.down.abs() <= stroke.across.abs() * Swipe::steepest().fraction()
            })
            .map(|stroke| {
                if stroke.across < Pixels::ZERO {
                    Toward::Next
                } else {
                    Toward::Previous
                }
            })
    }
}
