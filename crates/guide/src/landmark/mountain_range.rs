use crate::english_name::EnglishName;
use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::geo::{Coordinate, Share};
use crate::ruby_text::RubyText;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MountainRange {
    #[expect(dead_code, reason = "kept so the file loads")]
    source: Option<IgnoredAny>,
    #[expect(dead_code, reason = "kept so the file loads")]
    unverified: Option<IgnoredAny>,
    #[as_ref]
    name: RubyText,
    #[as_ref]
    english: EnglishName,
    from: Coordinate,
    to: Coordinate,
}

impl MountainRange {
    #[must_use]
    pub fn middle(&self) -> Coordinate {
        self.from.toward(self.to, Share::HALF)
    }
}
