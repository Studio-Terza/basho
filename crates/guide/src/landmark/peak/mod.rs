mod elevation;

use crate::english_name::EnglishName;
use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::evidence::Source;
use crate::geo::Coordinate;
use crate::ruby_text::RubyText;
pub use elevation::Elevation;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Peak {
    #[expect(dead_code, reason = "kept so the file loads")]
    list: Option<IgnoredAny>,
    #[expect(dead_code, reason = "kept so the file loads")]
    route_km: Option<IgnoredAny>,
    #[expect(dead_code, reason = "kept so the file loads")]
    source: Option<IgnoredAny>,
    #[as_ref]
    name: RubyText,
    #[as_ref]
    english: EnglishName,
    #[serde(rename = "meters")]
    elevation: Elevation,
    #[as_ref]
    at: Coordinate,
    #[as_ref]
    located: Source,
}

impl Peak {
    #[must_use]
    pub fn elevation(&self) -> Elevation {
        self.elevation
    }
}
