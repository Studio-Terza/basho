use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(newtype::Display)]
pub struct BuildNumber(u64);

impl BuildNumber {
    const MINUTE: Duration = Duration::from_secs(60);

    #[must_use]
    pub fn now() -> BuildNumber {
        BuildNumber(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |elapsed| elapsed.as_secs() / Self::MINUTE.as_secs()),
        )
    }
}
