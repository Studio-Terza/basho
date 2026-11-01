use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct UnixSeconds(u64);

impl From<SystemTime> for UnixSeconds {
    fn from(moment: SystemTime) -> UnixSeconds {
        UnixSeconds(
            moment
                .duration_since(UNIX_EPOCH)
                .map_or(0, |since| since.as_secs()),
        )
    }
}
