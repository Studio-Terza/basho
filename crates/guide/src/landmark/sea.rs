use crate::english_name::EnglishName;
use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::geo::Coordinate;
use crate::place::RubyText;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sea {
    #[expect(dead_code, reason = "kept so the file loads")]
    source: Option<IgnoredAny>,
    #[expect(dead_code, reason = "kept so the file loads")]
    unverified: Option<IgnoredAny>,
    #[as_ref]
    name: RubyText,
    #[as_ref]
    english: EnglishName,
    at: Coordinate,
}

impl AsRef<Coordinate> for Sea {
    fn as_ref(&self) -> &Coordinate {
        &self.at
    }
}
