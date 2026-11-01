use crate::english_name::EnglishName;
use serde::Deserialize;

use crate::clock::TripZone;
use crate::trip::language::Language;

#[derive(derive_more::AsRef, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TripHead {
    #[as_ref]
    name: EnglishName,
    utc_offset: TripZone,
    lang: Language,
}

impl TripHead {
    #[must_use]
    pub fn language(&self) -> Language {
        self.lang
    }

    #[must_use]
    pub fn zone(&self) -> TripZone {
        self.utc_offset
    }
}
