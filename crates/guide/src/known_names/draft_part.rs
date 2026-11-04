use std::fmt;

use serde::Serialize;

use crate::content::{NameKind, StretchKind, TypedStretch};
use crate::ruby_text::{LetterCount, LetterIndex, RubyText};
use crate::words::Spelling;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftPart {
    #[serde(rename = "entry")]
    Mention(Spelling),
    #[serde(rename = "place")]
    Spot(Spelling),
    #[serde(rename = "distance")]
    Length(Spelling),
    #[serde(rename = "unknown")]
    Unsure(Spelling),
    #[serde(untagged)]
    Words(Spelling),
}

impl DraftPart {
    #[must_use]
    pub fn named(kind: NameKind, name: &RubyText) -> DraftPart {
        match kind {
            NameKind::Entry => DraftPart::Mention(name.markup()),
            NameKind::Spot => DraftPart::Spot(name.markup()),
        }
    }

    #[must_use]
    pub fn letter_count(&self) -> LetterCount {
        RubyText::try_from(self.to_string())
            .map_or(LetterCount::default(), |text| LetterCount::from(&text))
    }

    fn words(markup: &Spelling, from: LetterIndex, to: LetterIndex) -> Option<DraftPart> {
        Some(from.between(to, markup))
            .filter(|text| !text.is_empty())
            .map(DraftPart::Words)
    }

    fn around(markup: &Spelling, stretch: &TypedStretch) -> Vec<DraftPart> {
        let indexes = stretch.index_range();
        let said = indexes.start.between(indexes.end, markup);
        let typed = match stretch.kind() {
            StretchKind::Clock => DraftPart::Unsure(said),
            StretchKind::Distance => DraftPart::Length(said),
        };
        Self::words(markup, LetterIndex::default(), indexes.start)
            .into_iter()
            .chain([typed])
            .chain(Self::split(&indexes.end.between(LetterIndex::END, markup)))
            .collect()
    }

    pub(super) fn split(markup: &Spelling) -> Vec<DraftPart> {
        markup.parse::<TypedStretch>().map_or_else(
            |_| {
                Self::words(markup, LetterIndex::default(), LetterIndex::END)
                    .into_iter()
                    .collect()
            },
            |stretch| Self::around(markup, &stretch),
        )
    }
}

impl fmt::Display for DraftPart {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DraftPart::Mention(text)
            | DraftPart::Spot(text)
            | DraftPart::Length(text)
            | DraftPart::Unsure(text)
            | DraftPart::Words(text) => formatter.write_str(text),
        }
    }
}
