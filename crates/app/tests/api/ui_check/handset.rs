use guide::plane::Pixels;
use layout::scale::Viewport;

pub struct Handset;

impl Handset {
    pub const WIDTH: Pixels = Pixels::new(430.0);
    #[cfg(not(test))]
    pub const HEIGHT: Pixels = Pixels::new(932.0);
    #[cfg(not(test))]
    pub const SCALE: Pixels = Pixels::new(3.0);
    #[cfg(not(test))]
    pub const SAFE_AREA: &str = ":root { --safe-top: 59px; --safe-bottom: 34px; }";

    pub fn viewport() -> Viewport {
        Viewport::default().at_width(Self::WIDTH)
    }
}
