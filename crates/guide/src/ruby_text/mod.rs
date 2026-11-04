mod boundary;
mod letters;
mod ruby_error;
mod segment;

use newtype::Counter;
use serde::Deserialize;
use std::ops::Range;

use crate::reading::Reading;
use crate::words::{Label, Spelling};
pub use letters::{Letter, LetterCount, LetterIndex, Writing};
use ruby_error::RubyError;
use segment::{AnnotatedPair, Segment};

struct Placed<'text> {
    span: Range<LetterIndex>,
    segment: &'text Segment,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct RubyText {
    segments: Vec<Segment>,
}

impl RubyText {
    #[must_use]
    pub fn verbatim(text: Spelling) -> RubyText {
        RubyText {
            segments: vec![Segment::Plain { text }],
        }
    }

    #[must_use]
    pub fn joined(pieces: impl Iterator<Item = RubyText>) -> RubyText {
        RubyText {
            segments: pieces
                .flat_map(|piece| piece.segments)
                .fold(Vec::new(), Segment::joined_onto),
        }
    }

    #[must_use]
    pub fn markup(&self) -> Spelling {
        Spelling::new(
            self.segments
                .iter()
                .map(|segment| {
                    segment.reading().map_or_else(
                        || segment.text().to_string(),
                        |reading| format!("{{{}|{reading}}}", segment.text()),
                    )
                })
                .collect::<String>(),
        )
    }

    #[must_use]
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    #[must_use]
    pub fn letters(&self) -> Vec<Letter> {
        self.text(Reading::Written)
            .chars()
            .map(Letter::from)
            .collect()
    }

    #[must_use]
    pub fn parts_at(&self, at: LetterIndex) -> Option<LetterIndex> {
        self.spans()
            .filter(|placed| placed.segment.reading().is_some())
            .find_map(|placed| at.inside(&placed.span))
            .map_or(Some(at), |_| None)
    }

    #[must_use]
    pub fn spelled_at(
        &self,
        letters: &[Letter],
        name: &[Letter],
        at: LetterIndex,
    ) -> Option<Range<LetterIndex>> {
        let end = at + LetterCount::new(name.len());
        letters
            .get(at.count()..end.count())
            .filter(|found| *found == name)
            .and_then(|_| self.parts_at(at))
            .and_then(|_| self.parts_at(end))
            .map(|_| at..end)
    }

    #[must_use]
    pub fn slice(&self, kept: &Range<LetterIndex>) -> RubyText {
        RubyText {
            segments: self
                .spans()
                .filter_map(|placed| placed.segment.within(placed.span.start, kept))
                .collect(),
        }
    }

    fn spans(&self) -> impl Iterator<Item = Placed<'_>> {
        self.segments
            .iter()
            .scan(LetterIndex::default(), |start, segment| {
                let first = *start;
                *start = first + segment.letter_count();
                Some(Placed {
                    span: first..*start,
                    segment,
                })
            })
    }

    #[must_use]
    pub fn ends_like(&self, other: &RubyText) -> bool {
        self.text(Reading::Written)
            .to_lowercase()
            .ends_with(&other.text(Reading::Written).to_lowercase())
    }

    #[must_use]
    pub fn text(&self, reading: Reading) -> Spelling {
        Spelling::new(
            self.segments
                .iter()
                .map(|segment| match reading {
                    Reading::Written => segment.text().as_ref(),
                    Reading::Spoken => segment.reading().unwrap_or(segment.text()).as_ref(),
                })
                .collect::<String>(),
        )
    }
}

impl From<Label> for RubyText {
    fn from(label: Label) -> RubyText {
        RubyText {
            segments: vec![Segment::Plain {
                text: Spelling::new(label.to_string()),
            }],
        }
    }
}

impl TryFrom<String> for RubyText {
    type Error = RubyError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        let lead = text
            .split('{')
            .take(1)
            .map(|piece| Segment::try_from(piece).map(|segment| vec![segment]));
        let annotated = text
            .split('{')
            .skip(1)
            .map(|piece| piece.parse::<AnnotatedPair>().map(Vec::from));
        lead.chain(annotated)
            .collect::<Result<Vec<Vec<Segment>>, RubyError>>()
            .map(|groups| RubyText {
                segments: groups
                    .into_iter()
                    .flatten()
                    .filter(|segment| !segment.text().is_empty())
                    .collect(),
            })
    }
}

impl From<&RubyText> for LetterCount {
    fn from(text: &RubyText) -> LetterCount {
        text.segments
            .iter()
            .map(Segment::letter_count)
            .fold(LetterCount::default(), LetterCount::plus)
    }
}
