use guide::geo::Share;
use guide::plane::Pixels;

use crate::closing::Closing;
use crate::pose::{Follow, Pose, Reach};
use crate::state::State;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Open;

impl State for Open {
    fn pose(self, _width: Pixels) -> Pose {
        Pose::new(Share::new(1.0), Pixels::ZERO, Follow::Ease, Reach::Live)
    }
}

impl Open {
    pub fn close(self) -> Closing {
        let Open = self;
        Closing
    }
}
