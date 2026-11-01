mod arrow_frame;

use crate::geo::Bearing;
use arrow_frame::ArrowFrame;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrowAngle {
    turn: Bearing,
    frame: ArrowFrame,
}

impl ArrowAngle {
    #[must_use]
    pub fn new(bearing: Bearing, heading: Option<Bearing>) -> ArrowAngle {
        heading.map_or(
            ArrowAngle {
                turn: bearing,
                frame: ArrowFrame::FromNorth,
            },
            |facing| ArrowAngle {
                turn: bearing.relative_to(facing),
                frame: ArrowFrame::FromHeading,
            },
        )
    }

    #[must_use]
    pub fn turn(self) -> Bearing {
        self.turn
    }

    #[must_use]
    pub fn frame(self) -> ArrowFrame {
        self.frame
    }
}
