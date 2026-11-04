use guide::plane::Pixels;
use guide::plane::Scale;

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
            MapSpace::ClearHalf => Self::total([Gap::Snug.design(), Gap::Hair.design()]),
            MapSpace::ControlFoot => Self::total([
                Gap::Tight.design(),
                Extent::Column.design(),
                Gap::Snug.design(),
            ]),
            MapSpace::ScaleWidth => {
                Self::total([Extent::ScaleBar.design(), Extent::ScaleText.design()])
            }
            MapSpace::NorthWidth => Self::total([
                Extent::Glyph.design(),
                Gap::Tight.design() * Self::BOTH_SIDES.factor(),
                Gap::Snug.design() * Self::BOTH_SIDES.factor(),
            ]),
            MapSpace::NorthHeight => Self::total([
                Extent::Column.design(),
                Extent::Glyph.design(),
                Gap::Tight.design() * Self::BOTH_SIDES.factor(),
            ]),
            MapSpace::ShieldSpacing => Self::total([
                Gap::Open.design() * Self::SHIELD_GAPS.factor(),
                Gap::Gutter.design(),
            ]),
            MapSpace::PointerSide => Self::total([Gap::Thin.design(), Gap::Hair.design()]),
            MapSpace::PointerHalfTall => Self::total([Gap::Close.design(), Gap::Hair.design()]),
        }
    }

    fn total(parts: impl IntoIterator<Item = Pixels>) -> Pixels {
        parts
            .into_iter()
            .fold(Pixels::ZERO, |length, part| length + part)
    }
}
