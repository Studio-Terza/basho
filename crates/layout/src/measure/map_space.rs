use guide::map_frame::{Pixels, Scale};

use crate::measure::Extent;
use crate::scale::{Gap, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapSpace {
    ClearHalf,
    ControlFoot,
    ScaleWidth,
    NorthWidth,
    NorthHeight,
    ShieldSpacing,
    PointerSide,
    PointerHalfTall,
}

impl MapSpace {
    const BOTH_SIDES: Scale = Scale::new(2.0);
    const SHIELD_GAPS: Scale = Scale::new(3.0);

    #[must_use]
    pub fn length(self) -> Pixels {
        match self {
            MapSpace::ClearHalf => Gap::Snug.design() + Gap::Hair.design(),
            MapSpace::ControlFoot => {
                Gap::Tight.design() + Extent::Column.design() + Gap::Snug.design()
            }
            MapSpace::ScaleWidth => Extent::ScaleBar.design() + Extent::ScaleText.design(),
            MapSpace::NorthWidth => {
                Extent::Glyph.design()
                    + Gap::Tight.design() * Self::BOTH_SIDES.factor()
                    + Gap::Snug.design() * Self::BOTH_SIDES.factor()
            }
            MapSpace::NorthHeight => {
                Extent::Column.design()
                    + Extent::Glyph.design()
                    + Gap::Tight.design() * Self::BOTH_SIDES.factor()
            }
            MapSpace::ShieldSpacing => {
                Gap::Open.design() * Self::SHIELD_GAPS.factor() + Gap::Gutter.design()
            }
            MapSpace::PointerSide => Gap::Thin.design() + Gap::Hair.design(),
            MapSpace::PointerHalfTall => Gap::Close.design() + Gap::Hair.design(),
        }
    }
}
