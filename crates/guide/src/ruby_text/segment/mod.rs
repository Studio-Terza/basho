mod annotated_pair;
mod script;

use crate::ruby_text::ruby_error::RubyError;
use crate::ruby_text::{LetterCount, LetterIndex};
use crate::words::{Spelling, Written};
pub use annotated_pair::AnnotatedPair;
use newtype::Counter;
use script::Script;
use std::ops::Range;

#[derive(Clone, Debug)]
pub enum Segment {
    Plain { text: Spelling },
    Ruby { base: Spelling, reading: Spelling },
}

impl Segment {
    #[must_use]
    pub fn joined_onto(kept: Vec<Segment>, next: Segment) -> Vec<Segment> {
        let open = matches!(kept.last(), Some(Segment::Plain { .. }));
        match (open, &next) {
            (_, Segment::Plain { text }) if text.is_empty() => kept,
            (true, Segment::Plain { text }) => kept
                .iter()
                .take(kept.len() - 1)
                .cloned()
                .chain(kept.last().map(|before| Segment::Plain {
                    text: Spelling::new(format!("{}{text}", before.text())),
                }))
                .collect(),
            _ => kept.into_iter().chain([next]).collect(),
        }
    }

    fn plain(text: &str) -> Result<Segment, RubyError> {
        if text
            .chars()
            .map(Script::from)
            .any(|script| script == Script::Kanji)
        {
            Err(RubyError::UnreadKanji(Written::new(text.to_owned())))
        } else if text.contains(['}', '|']) {
            Err(RubyError::StrayMarkup(Written::new(text.to_owned())))
        } else {
            Ok(Segment::Plain {
                text: Spelling::new(text.to_owned()),
            })
        }
    }

    #[must_use]
    pub fn letter_count(&self) -> LetterCount {
        LetterCount::new(self.text().chars().count())
    }

    #[must_use]
    pub fn within(&self, first: LetterIndex, kept: &Range<LetterIndex>) -> Option<Segment> {
        let kept: String = self
            .text()
            .chars()
            .skip(kept.start.count().saturating_sub(first.count()))
            .take(
                kept.end
                    .count()
                    .saturating_sub(first.max(kept.start).count()),
            )
            .collect();
        kept.chars().next().map(|_| self.kept(Spelling::new(kept)))
    }

    fn kept(&self, text: Spelling) -> Segment {
        match self {
            Segment::Plain { .. } => Segment::Plain { text },
            Segment::Ruby { .. } => self.clone(),
        }
    }

    #[must_use]
    pub fn text(&self) -> &Spelling {
        match self {
            Segment::Plain { text } => text,
            Segment::Ruby { base, .. } => base,
        }
    }

    #[must_use]
    pub fn reading(&self) -> Option<&Spelling> {
        match self {
            Segment::Plain { .. } => None,
            Segment::Ruby { reading, .. } => Some(reading),
        }
    }
}

impl TryFrom<&str> for Segment {
    type Error = RubyError;

    fn try_from(text: &str) -> Result<Segment, RubyError> {
        Segment::plain(text)
    }
}
