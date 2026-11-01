use guide::geo::Share;
use guide::map_frame::Pixels;

use crate::open::Open;
use crate::pose::{Follow, Pose, Reach};
use crate::state::State;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Closed;

impl State for Closed {
    fn pose(self, width: Pixels) -> Pose {
        Pose::new(Share::new(0.0), -width, Follow::Ease, Reach::Inert)
    }
}

impl Closed {
    pub fn open(self) -> Open {
        let Closed = self;
        Open
    }
}
