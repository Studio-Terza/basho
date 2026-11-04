mod ruby_error;
mod segment;

use serde::Deserialize;

use crate::reading::Reading;
use crate::words::{Label, Spelling};
use ruby_error::RubyError;
use segment::{AnnotatedPair, Segment};

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
    pub fn char_length(&self) -> usize {
        self.segments
            .iter()
            .map(|segment| segment.text().chars().count())
            .sum()
    }

    #[must_use]
    pub fn letters(&self) -> Vec<char> {
        self.text(Reading::Written).chars().collect()
    }

    #[must_use]
    pub fn parts_at(&self, at_index: usize) -> Option<usize> {
        self.segments
            .iter()
            .scan(0, |start_index, segment| {
                let length = segment.text().chars().count();
                let first_index = *start_index;
                *start_index += length;
                Some(
                    segment
                        .reading()
                        .filter(|_| (first_index + 1..first_index + length).contains(&at_index)),
                )
            })
            .flatten()
            .next()
            .is_none()
            .then_some(at_index)
    }

    #[must_use]
    pub fn slice(&self, from_index: usize, to_index: usize) -> RubyText {
        RubyText {
            segments: self
                .segments
                .iter()
                .scan(0, |start_index, segment| {
                    let first_index = *start_index;
                    *start_index += segment.text().chars().count();
                    Some(segment.within(first_index, from_index, to_index))
                })
                .flatten()
                .collect(),
        }
    }

    #[must_use]
    pub fn ends_like(&self, other: &RubyText) -> Option<&RubyText> {
        Some(self).filter(|text| {
            text.text(Reading::Written)
                .to_lowercase()
                .ends_with(&other.text(Reading::Written).to_lowercase())
        })
    }

    #[must_use]
    pub fn text(&self, reading: Reading) -> Spelling {
        Spelling::new(
            self.segments
                .iter()
                .map(|segment| segment.shown(reading).as_ref())
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
                    .filter(|segment| segment.text().chars().next().is_some())
                    .collect(),
            })
    }
}
