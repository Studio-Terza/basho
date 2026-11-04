use std::str::FromStr;

use tile_draft::{TagKey, TileTag};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KanaReading(String);

impl KanaReading {
    const KATAKANA: std::ops::RangeInclusive<char> = '\u{30a1}'..='\u{30f6}';
    const HIRAGANA: std::ops::RangeInclusive<char> = '\u{3041}'..='\u{3096}';
    const LONG_MARK: char = '\u{30fc}';

    pub(super) fn joined(parts: Vec<KanaReading>) -> Option<KanaReading> {
        let text: String = parts.into_iter().map(|part| part.0).collect();
        (!text.is_empty()).then_some(KanaReading(text))
    }

    pub(super) fn softened(text: &str) -> KanaReading {
        KanaReading(text.chars().map(Self::hiragana).collect())
    }

    pub(super) fn tag(self, key: TagKey) -> TileTag {
        TileTag::new(key, self.0.as_str().into())
    }

    fn is_kana(letter: char) -> bool {
        Self::KATAKANA.contains(&letter)
            || Self::HIRAGANA.contains(&letter)
            || letter == Self::LONG_MARK
            || letter == ' '
    }

    fn hiragana(letter: char) -> char {
        Self::KATAKANA
            .zip(Self::HIRAGANA)
            .find(|(katakana, _)| *katakana == letter)
            .map_or(letter, |(_, hiragana)| hiragana)
    }
}

impl FromStr for KanaReading {
    type Err = ();

    fn from_str(text: &str) -> Result<KanaReading, ()> {
        let trimmed = text.trim();
        if !trimmed.is_empty() && trimmed.chars().all(Self::is_kana) {
            Ok(Self::softened(trimmed))
        } else {
            Err(())
        }
    }
}
