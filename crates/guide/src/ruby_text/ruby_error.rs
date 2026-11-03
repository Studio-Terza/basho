use crate::words::Written;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RubyError {
    #[error("kanji without furigana in {0:?}; write {{kanji|reading}}")]
    UnreadKanji(Written),
    #[error("ruby opened with {{ but never closed in {0:?}")]
    UnclosedRuby(Written),
    #[error("ruby {0:?} has no |reading")]
    MissingReading(Written),
    #[error("reading {0:?} must be kana only")]
    ReadingNotKana(Written),
    #[error("stray ruby markup in {0:?}")]
    StrayMarkup(Written),
}
