use guide::map_frame::Pixels;

use crate::pose::Pose;

pub trait State: Copy {
    fn pose(self, width: Pixels) -> Pose;
}
