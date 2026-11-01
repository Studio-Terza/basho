use guide::geo::{Degrees, Share};
use guide::map_frame::{Pixels, Scale};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Touch {
    Tap,
    EdgeZone,
    Slop,
}

impl Touch {
    const FINGERTIP: Pixels = Pixels::new(44.0);
    const STEEPEST: Degrees = Degrees::new(40.0);
    const TAP: Scale = Scale::new(1.0);
    const EDGE_ZONE: Scale = Scale::new(0.6);
    const SLOP: Scale = Scale::new(0.23);

    #[must_use]
    pub fn extent(self) -> Pixels {
        Self::FINGERTIP
            * match self {
                Touch::Tap => Self::TAP,
                Touch::EdgeZone => Self::EDGE_ZONE,
                Touch::Slop => Self::SLOP,
            }
            .factor()
    }

    #[must_use]
    pub fn steepest() -> Share {
        let steepest = Self::STEEPEST.degrees().to_radians();
        Share::ratio(steepest.sin(), steepest.cos())
    }
}
