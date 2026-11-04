mod kana_reading;
mod written;

use tile_draft::{TagKey, TileTag};

use crate::osm_tags::{OsmKey, OsmTags};
use kana_reading::KanaReading;
use written::Written;

pub struct Labels<'tags>(&'tags OsmTags);

impl<'tags> Labels<'tags> {
    pub fn new(tags: &'tags OsmTags) -> Labels<'tags> {
        Labels(tags)
    }

    pub fn tags(&self) -> Vec<TileTag> {
        let named = self.0.text(OsmKey::NAME).map(|name| {
            name.parse::<Written>().map_or_else(
                |()| vec![Self::label(name)],
                |written| self.named(name, &written),
            )
        });
        self.plain(&OsmKey::LATIN)
            .or(named)
            .or_else(|| self.plain(&OsmKey::KANA))
            .unwrap_or_default()
    }

    fn plain(&self, keys: &[OsmKey]) -> Option<Vec<TileTag>> {
        keys.iter()
            .filter_map(|key| self.0.text(*key))
            .find(|text| text.parse::<Written>().is_err())
            .map(|text| vec![Self::label(text)])
    }

    fn named(&self, name: &str, written: &Written) -> Vec<TileTag> {
        self.reading(written)
            .map(|reading| vec![Self::label(name), reading])
            .unwrap_or_default()
    }

    fn reading(&self, written: &Written) -> Option<TileTag> {
        OsmKey::KANA
            .into_iter()
            .find_map(|key| self.0.parsed::<KanaReading>(key))
            .map(|reading| reading.tag(TagKey::Reading))
            .or_else(|| written.reading().map(|reading| reading.tag(TagKey::Guess)))
    }

    fn label(text: &str) -> TileTag {
        TileTag::new(TagKey::Label, text.into())
    }
}
