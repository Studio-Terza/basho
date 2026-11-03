mod backing;
mod platform_number;
pub use backing::Backing;

use serde::Deserialize;
use std::fmt;

use platform_number::PlatformNumber;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Platform {
    Published {
        number: PlatformNumber,
        evidence: Backing,
    },
    NotPublished,
}

impl Platform {
    #[must_use]
    pub fn evidence(&self) -> Option<&Backing> {
        match self {
            Platform::Published { evidence, .. } => Some(evidence),
            Platform::NotPublished => None,
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::Published { number, .. } => number.fmt(formatter),
            Platform::NotPublished => {
                formatter.write_str("Platform not published: check the board")
            }
        }
    }
}
