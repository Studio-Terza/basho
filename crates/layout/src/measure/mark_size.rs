use guide::classed::Classed;
use guide::geo::{Degrees, Share};
use guide::map_frame::{Pixels, Scale};

use crate::scale::Gap;
use crate::scale::Token;

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkSize {
    #[strum(serialize = "tiny")]
    Tiny,
    #[strum(serialize = "small-mark")]
    Small,
    #[strum(serialize = "large")]
    Large,
}

impl MarkSize {
    pub const STAR_INNER: Share = Share::new(0.45);
    pub const CORE: Share = Share::new(0.4);
    pub const YOU_DOT: Share = Share::new(0.5);
    pub const PEAK_DROP: Share = Share::new(0.75);
    const STAR_STEP: Degrees = Degrees::new(36.0);
    const FULL_TURN: Degrees = Degrees::new(360.0);
    const QUARTER: Degrees = Degrees::new(90.0);
    const TINY: Pixels = Pixels::new(3.0);
    const SMALL: Pixels = Pixels::new(4.0);
    const LARGE: Pixels = Pixels::new(6.5);
    const TRAIL_DOT_SHARE: Share = Share::new(0.5);
    const CONE_REACH_SCALE: Scale = Scale::new(3.0);
    pub const TRAIL_DOT: Pixels =
        Pixels::new(Self::TINY.pixels() * Self::TRAIL_DOT_SHARE.fraction());
    pub const CONE_REACH: Pixels =
        Pixels::new(Self::LARGE.pixels() * Self::CONE_REACH_SCALE.factor());

    #[must_use]
    pub fn outline_reach() -> Pixels {
        Gap::Hair.design()
    }

    #[must_use]
    pub fn star_corners() -> Vec<Degrees> {
        std::iter::successors(Some(Degrees::new(0.0)), |angle| {
            Some(*angle + Self::STAR_STEP)
        })
        .take_while(|angle| angle.degrees() < Self::FULL_TURN.degrees())
        .map(|angle| angle - Self::QUARTER)
        .collect()
    }

    #[must_use]
    pub fn half(self) -> Pixels {
        match self {
            MarkSize::Tiny => Self::TINY,
            MarkSize::Small => Self::SMALL,
            MarkSize::Large => Self::LARGE,
        }
    }
}

impl Classed for MarkSize {}
