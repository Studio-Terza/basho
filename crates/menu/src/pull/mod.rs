mod drag_from;

use guide::map_frame::{Axis, Pixels, PlanePoint};
use layout::Panel;
use layout::scale::Viewport;

pub use drag_from::DragFrom;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pull {
    reach: Pixels,
}

impl Pull {
    #[must_use]
    pub fn dragged(
        from: DragFrom,
        start: PlanePoint,
        latest: PlanePoint,
        viewport: Viewport,
    ) -> Pull {
        let width = Panel::Menu.width(viewport);
        let travel = latest[Axis::Across] - start[Axis::Across];
        Pull {
            reach: (from.rest(width) + travel).clamp(Pixels::ZERO, width),
        }
    }

    #[must_use]
    pub fn reach(self) -> Pixels {
        self.reach
    }
}
