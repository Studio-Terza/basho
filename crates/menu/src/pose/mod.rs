mod follow;
mod reach;
use std::fmt;
use std::time::Duration;

use guide::map_frame::Pixels;

use guide::geo::Share;

pub use follow::Follow;
pub use reach::Reach;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    shade: Share,
    offset: Pixels,
    follow: Follow,
    reach: Reach,
}

impl Pose {
    pub fn new(shade: Share, offset: Pixels, follow: Follow, reach: Reach) -> Pose {
        Pose {
            shade,
            offset,
            follow,
            reach,
        }
    }

    pub fn held(pull: Pixels, width: Pixels) -> Pose {
        Pose::new(
            Share::ratio(pull, width),
            pull - width,
            Follow::Finger,
            Reach::Live,
        )
    }

    pub fn styled(self, rest: Pose, width: Pixels) -> PoseStyle {
        PoseStyle {
            now: self,
            rest,
            width,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PoseStyle {
    now: Pose,
    rest: Pose,
    width: Pixels,
}

impl fmt::Display for PoseStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PoseStyle { now, rest, width } = *self;
        write!(
            formatter,
            "--menu-width: {width}px; --shade: {:.3}; --offset: {}px; --follow: {}ms; --rest-shade: {:.3}; --rest-offset: {}px; pointer-events: {}",
            now.shade.fraction(),
            now.offset,
            Duration::from(now.follow).as_millis(),
            rest.shade.fraction(),
            rest.offset,
            now.reach.pointer_events(),
        )
    }
}
