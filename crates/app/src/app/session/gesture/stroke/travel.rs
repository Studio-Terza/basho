use crate::app::session::gesture::Gesture;
use crate::app::session::gesture::origin::Origin;
use crate::app::session::gesture::touch_sample::TouchSample;
use crate::app::session::hand::Flick;

use super::Stroke;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Travel {
    Still,
    Level(Stroke),
    Steep,
}

impl Travel {
    pub fn gesture_after(
        self,
        pressed: Gesture,
        origin: Origin,
        start: TouchSample,
        sample: TouchSample,
    ) -> Gesture {
        match self {
            Travel::Still => pressed,
            Travel::Level(level) => Gesture::Dragging {
                origin,
                start,
                latest: sample,
                flick: Flick::default().followed(level.across(), start.time(), sample.time()),
            },
            Travel::Steep => Gesture::Scrolling,
        }
    }

    pub fn pair_after(self, held: Gesture) -> Gesture {
        match self {
            Travel::Still => held,
            Travel::Level(_) | Travel::Steep => Gesture::Scrolling,
        }
    }
}
