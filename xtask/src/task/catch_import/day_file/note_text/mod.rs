use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::fmt;

mod piece;

use guide::readout::recovery::catch_words::Readings;
use guide::ruby_text::RubyText;
use guide::trip::{DraftPart, KnownNames};
use piece::Piece;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoteText {
    pieces: Vec<Piece>,
}

impl From<&str> for NoteText {
    fn from(text: &str) -> NoteText {
        NoteText {
            pieces: Piece::parsed(text),
        }
    }
}

impl NoteText {
    #[must_use]
    pub fn parts(&self, names: &KnownNames) -> Option<Vec<DraftPart>> {
        RubyText::try_from(self.to_string())
            .ok()
            .map(|note| names.drafted(&note))
    }

    #[must_use]
    pub fn annotated(self, readings: &Readings) -> NoteText {
        readings
            .iter()
            .filter(|(name, _)| !AsRef::<str>::as_ref(*name).is_empty())
            .map(|(name, kana)| (Reverse(AsRef::<str>::as_ref(name).len()), name, kana))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .fold(self, |note, (_, name, kana)| NoteText {
                pieces: note
                    .pieces
                    .into_iter()
                    .flat_map(|piece| piece.with_reading(name, kana))
                    .collect(),
            })
    }
}

impl fmt::Display for NoteText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pieces
            .iter()
            .try_for_each(|piece| write!(formatter, "{piece}"))
    }
}
