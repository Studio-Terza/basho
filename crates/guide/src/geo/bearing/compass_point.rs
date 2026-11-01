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
        Bearing::new(
            match self {
                CompassPoint::N => Self::NORTH,
                CompassPoint::NE => Self::NORTH_EAST,
                CompassPoint::E => Self::EAST,
                CompassPoint::SE => Self::SOUTH_EAST,
                CompassPoint::S => Self::SOUTH,
                CompassPoint::SW => Self::SOUTH_WEST,
                CompassPoint::W => Self::WEST,
                CompassPoint::NW => Self::NORTH_WEST,
            }
            .degrees(),
        )
    }

    #[must_use]
    pub fn words(self) -> Label {
        Label::new(match self {
            CompassPoint::N => "NORTH",
            CompassPoint::NE => "NORTH-EAST",
            CompassPoint::E => "EAST",
            CompassPoint::SE => "SOUTH-EAST",
            CompassPoint::S => "SOUTH",
            CompassPoint::SW => "SOUTH-WEST",
            CompassPoint::W => "WEST",
            CompassPoint::NW => "NORTH-WEST",
        })
    }
}
