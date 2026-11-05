use std::time::Duration;

use crate::Motion;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Follow {
    Finger,
    Ease,
}

impl From<Follow> for Duration {
    fn from(follow: Follow) -> Duration {
        match follow {
            Follow::Finger => Duration::ZERO,
            Follow::Ease => Duration::from(Motion::Slide),
        }
    }
}
