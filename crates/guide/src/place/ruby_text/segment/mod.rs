mod annotated_pair;
mod script;

use crate::place::ruby_text::ruby_error::RubyError;
use crate::words::{Spelling, Written};
pub use annotated_pair::AnnotatedPair;
use script::Script;

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
    pub fn within(
        &self,
        first_index: usize,
        from_index: usize,
        to_index: usize,
    ) -> Option<Segment> {
        let kept: String = self
            .text()
            .chars()
            .skip(from_index.saturating_sub(first_index))
            .take(to_index.saturating_sub(first_index.max(from_index)))
            .collect();
        match self {
            _ if kept.is_empty() => None,
            Segment::Plain { .. } => Some(Segment::Plain {
                text: Spelling::new(kept),
            }),
            Segment::Ruby { .. } => Some(self.clone()),
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
