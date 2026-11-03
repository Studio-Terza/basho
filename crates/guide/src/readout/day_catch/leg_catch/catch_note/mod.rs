mod note_text;

use crate::readout::day_catch::FeatureId;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

use crate::content::Content;
use note_text::NoteText;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatchNote {
    feature: FeatureId,
    #[serde(default, deserialize_with = "CatchNote::readable")]
    ste: Option<Content>,
}

impl CatchNote {
    #[must_use]
    pub fn feature(&self) -> &FeatureId {
        &self.feature
    }

    #[must_use]
    pub fn ste(&self) -> Option<&Content> {
        self.ste.as_ref()
    }

    fn readable<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Content>, D::Error> {
        Content::deserialize(deserializer).and_then(|text| {
            NoteText::try_from(text)
                .map(|note| Some(note.content()))
                .map_err(D::Error::custom)
        })
    }
}
