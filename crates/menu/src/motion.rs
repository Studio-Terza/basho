use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Motion {
    Slide,
    Rest,
}

impl Motion {
    const SLIDE: Duration = Duration::from_millis(320);
    const REST: Duration = Duration::from_millis(100);
}

impl From<Motion> for Duration {
    fn from(motion: Motion) -> Duration {
        match motion {
            Motion::Slide => Motion::SLIDE,
            Motion::Rest => Motion::REST,
        }
    }
}
