use guide::geo::Share;
use guide::plane::Pixels;

use crate::closed::Closed;
use crate::open::Open;
use crate::pose::{Follow, Pose, Reach};
use crate::state::State;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Closing;

impl State for Closing {
    fn pose(self, width: Pixels) -> Pose {
        Pose::new(Share::new(0.0), -width, Follow::Ease, Reach::Inert)
    }
}

impl Closing {
    pub fn settle(self) -> Closed {
        let Closing = self;
        Closed
    }

    pub fn reopen(self) -> Open {
        let Closing = self;
        Open
    }
}
