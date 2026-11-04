use crate::words::Label;
use strum::{Display, EnumIter};

use super::Bearing;
use crate::geo::degrees::Degrees;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumIter)]
pub enum CompassPoint {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl CompassPoint {
    const NORTH: Degrees = Degrees::new(0.0);
    const NORTH_EAST: Degrees = Degrees::new(45.0);
    const EAST: Degrees = Degrees::new(90.0);
    const SOUTH_EAST: Degrees = Degrees::new(135.0);
    const SOUTH: Degrees = Degrees::new(180.0);
    const SOUTH_WEST: Degrees = Degrees::new(225.0);
    const WEST: Degrees = Degrees::new(270.0);
    const NORTH_WEST: Degrees = Degrees::new(315.0);

    #[must_use]
    pub fn center(self) -> Bearing {
        match self {
            CompassPoint::N => Bearing::new(Self::NORTH.degrees()),
            CompassPoint::NE => Bearing::new(Self::NORTH_EAST.degrees()),
            CompassPoint::E => Bearing::new(Self::EAST.degrees()),
            CompassPoint::SE => Bearing::new(Self::SOUTH_EAST.degrees()),
            CompassPoint::S => Bearing::new(Self::SOUTH.degrees()),
            CompassPoint::SW => Bearing::new(Self::SOUTH_WEST.degrees()),
            CompassPoint::W => Bearing::new(Self::WEST.degrees()),
            CompassPoint::NW => Bearing::new(Self::NORTH_WEST.degrees()),
        }
    }

    #[must_use]
    pub fn words(self) -> Label {
        match self {
            CompassPoint::N => Label::new("NORTH"),
            CompassPoint::NE => Label::new("NORTH-EAST"),
            CompassPoint::E => Label::new("EAST"),
            CompassPoint::SE => Label::new("SOUTH-EAST"),
            CompassPoint::S => Label::new("SOUTH"),
            CompassPoint::SW => Label::new("SOUTH-WEST"),
            CompassPoint::W => Label::new("WEST"),
            CompassPoint::NW => Label::new("NORTH-WEST"),
        }
    }
}
