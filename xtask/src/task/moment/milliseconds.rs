use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct Milliseconds(u64);

impl From<Duration> for Milliseconds {
    fn from(elapsed: Duration) -> Milliseconds {
        Milliseconds(u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX))
    }
}
