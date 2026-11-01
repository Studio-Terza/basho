use serde::Deserialize;
use std::fmt;
use std::num::NonZeroU8;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(transparent)]
pub struct PlatformNumber(NonZeroU8);

impl fmt::Display for PlatformNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Platform {}", self.0)
    }
}
