use guide::plane::Pixels;

use crate::pose::Pose;

pub trait State: Copy {
    fn pose(self, width: Pixels) -> Pose;
}
