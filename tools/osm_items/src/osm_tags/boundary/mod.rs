use plain_text::PlainText;
use serde::Deserialize;
use serde::de::IgnoredAny;

use std::collections::BTreeMap;

use crate::osm_tags::OsmKey;

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum OsmText {
    Text(PlainText),
    Other(IgnoredAny),
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(transparent)]
pub struct OsmTags(BTreeMap<PlainText, OsmText>);

impl OsmTags {
    pub(crate) fn text(&self, key: OsmKey) -> Option<&str> {
        match self.0.get(key.0)? {
            OsmText::Text(text) => Some(text.as_ref()),
            OsmText::Other(_) => None,
        }
    }
}
