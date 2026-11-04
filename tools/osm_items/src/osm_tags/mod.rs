use std::collections::BTreeMap;
use std::fmt;

use plain_text::PlainText;
use serde::Deserialize;
use serde::de::IgnoredAny;

use tile_draft::{TagKey, TileTag};

mod boundary;

use boundary::Labels;

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum OsmText {
    Text(PlainText),
    Other(IgnoredAny),
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(transparent)]
pub struct OsmTags(BTreeMap<PlainText, OsmText>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OsmKey(&'static str);

impl OsmKey {
    pub const AERIALWAY: OsmKey = OsmKey("aerialway");
    pub const AMENITY: OsmKey = OsmKey("amenity");
    pub const BRIDGE: OsmKey = OsmKey("bridge");
    pub const BUILDING: OsmKey = OsmKey("building");
    pub const HIGHWAY: OsmKey = OsmKey("highway");
    pub const HISTORIC: OsmKey = OsmKey("historic");
    pub const LANDUSE: OsmKey = OsmKey("landuse");
    pub const LEISURE: OsmKey = OsmKey("leisure");
    pub const NATURAL: OsmKey = OsmKey("natural");
    pub const PLACE: OsmKey = OsmKey("place");
    pub const RAILWAY: OsmKey = OsmKey("railway");
    pub const REF: OsmKey = OsmKey("ref");
    pub const ROUTE: OsmKey = OsmKey("route");
    pub const SHOP: OsmKey = OsmKey("shop");
    pub const SURFACE: OsmKey = OsmKey("surface");
    pub const TOURISM: OsmKey = OsmKey("tourism");
    pub const TUNNEL: OsmKey = OsmKey("tunnel");
    pub const WATERWAY: OsmKey = OsmKey("waterway");
    const LATIN: [OsmKey; 4] = [
        OsmKey("name:en"),
        OsmKey("brand:en"),
        OsmKey("name:ja-Latn"),
        OsmKey("name:ja_rm"),
    ];
    const NAME: OsmKey = OsmKey("name");
    const KANA: [OsmKey; 3] = [
        OsmKey("name:ja-Hira"),
        OsmKey("name:ja_kana"),
        OsmKey("name:ja-Kana"),
    ];
}

impl OsmTags {
    const ABSENT: &str = "no";

    pub(crate) fn text(&self, key: OsmKey) -> Option<&str> {
        match self.0.get(key.0)? {
            OsmText::Text(text) => Some(text.as_ref()),
            OsmText::Other(_) => None,
        }
    }

    pub fn parsed<Tag: std::str::FromStr>(&self, key: OsmKey) -> Option<Tag> {
        self.text(key)?.parse().ok()
    }

    pub fn tag_of(&self, key: TagKey) -> Option<TileTag> {
        match key {
            TagKey::Class | TagKey::Surface | TagKey::Label | TagKey::Reading | TagKey::Guess => {
                None
            }
            TagKey::Structure => self
                .structure()
                .map(|kind| TileTag::new(key, kind.0.into())),
            TagKey::Ref => self
                .text(OsmKey::REF)
                .map(|text| TileTag::new(key, text.into())),
        }
    }

    fn structure(&self) -> Option<OsmKey> {
        [OsmKey::BRIDGE, OsmKey::TUNNEL]
            .into_iter()
            .find(|kind| self.text(*kind).is_some_and(|value| value != Self::ABSENT))
    }

    pub fn label_tags(&self) -> Vec<TileTag> {
        Labels::new(self).tags()
    }
}

impl fmt::Display for OsmKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
