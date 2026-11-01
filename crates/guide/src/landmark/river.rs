use crate::english_name::EnglishName;
use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::geo::Coordinate;
use crate::place::RubyText;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct River {
    #[expect(dead_code, reason = "kept so the file loads")]
    osm: Option<IgnoredAny>,
    #[expect(dead_code, reason = "kept so the file loads")]
    source: Option<IgnoredAny>,
    #[expect(dead_code, reason = "kept so the file loads")]
    unverified: Option<IgnoredAny>,
    #[as_ref]
    name: RubyText,
    #[as_ref]
    english: EnglishName,
    labels: Vec<Coordinate>,
}

impl River {
    #[must_use]
    pub fn labels(&self) -> &[Coordinate] {
        &self.labels
    }
}
