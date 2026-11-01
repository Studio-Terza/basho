use std::fmt;

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum TagKey {
    Class,
    Surface,
    Structure,
    Ref,
    Label,
    Reading,
    Guess,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagText(String);

impl From<&str> for TagText {
    fn from(text: &str) -> TagText {
        TagText(text.to_owned())
    }
}

impl AsRef<[u8]> for TagText {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileTag {
    key: TagKey,
    text: TagText,
}

impl TileTag {
    #[must_use]
    pub fn new(key: TagKey, text: TagText) -> TileTag {
        TileTag { key, text }
    }

    #[must_use]
    pub fn key(&self) -> TagKey {
        self.key
    }

    #[must_use]
    pub fn text(&self) -> &TagText {
        &self.text
    }
}

impl fmt::Display for TileTag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}={}", <&str>::from(self.key), self.text.0)
    }
}
