mod annotated_pair;
mod script;

use crate::reading::Reading;
use crate::ruby_text::ruby_error::RubyError;
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
        match &next {
            Segment::Plain { text } => Self::plain_onto(&kept, text),
            Segment::Ruby { .. } => kept.into_iter().chain([next]).collect(),
        }
    }

    fn plain_onto(kept: &[Segment], text: &Spelling) -> Vec<Segment> {
        let open = kept
            .last()
            .filter(|last| matches!(last, Segment::Plain { .. }));
        let base = kept.len() - open.map_or(0, |_| 1);
        let tail = Segment::Plain {
            text: Spelling::new(format!(
                "{}{text}",
                open.map_or(String::new(), |before| before.text().to_string())
            )),
        };
        text.chars().next().map_or_else(
            || kept.to_vec(),
            |_| kept.iter().take(base).cloned().chain([tail]).collect(),
        )
    }

    fn plain(text: &str) -> Result<Segment, RubyError> {
        text.chars()
            .map(Script::from)
            .find(|script| *script == Script::Kanji)
            .map(|_| RubyError::UnreadKanji(Written::new(text.to_owned())))
            .or_else(|| {
                text.contains(['}', '|'])
                    .then(|| RubyError::StrayMarkup(Written::new(text.to_owned())))
            })
            .map_or_else(
                || {
                    Ok(Segment::Plain {
                        text: Spelling::new(text.to_owned()),
                    })
                },
                Err,
            )
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
        kept.chars().next().map(|_| {
            self.reading()
                .map_or_else(|| Segment::from(Spelling::new(kept)), |_| self.clone())
        })
    }

    #[must_use]
    pub fn text(&self) -> &Spelling {
        match self {
            Segment::Plain { text } => text,
            Segment::Ruby { base, .. } => base,
        }
    }

    #[must_use]
    pub fn shown(&self, reading: Reading) -> &Spelling {
        match reading {
            Reading::Written => self.text(),
            Reading::Spoken => self.reading().unwrap_or(self.text()),
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

impl From<Spelling> for Segment {
    fn from(text: Spelling) -> Segment {
        Segment::Plain { text }
    }
}
