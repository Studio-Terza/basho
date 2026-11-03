use guide::plane::Pixels;
use guide::words::Label;

use crate::scale::viewport::Viewport;

pub trait Token: Copy + 'static {
    const ALL: &'static [Self];

    fn name(self) -> Label;

    fn length(self, viewport: Viewport) -> Pixels;

    fn design(self) -> Pixels {
        self.length(Viewport::default())
    }
}
