use std::fmt;

use serde::Serialize;

use crate::content::{NameKind, StretchKind, TypedStretch};
use crate::reading::Reading;
use crate::ruby_text::RubyText;
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
    pub fn letter_length(&self) -> usize {
        RubyText::try_from(self.to_string())
            .map_or(0, |text| text.text(Reading::Written).chars().count())
    }

    fn words(text: &str) -> Option<DraftPart> {
        text.chars()
            .next()
            .map(|_| DraftPart::Words(Spelling::new(text.to_owned())))
    }

    fn typed(kind: StretchKind, said: Spelling) -> DraftPart {
        match kind {
            StretchKind::Clock => DraftPart::Unsure(said),
            StretchKind::Distance => DraftPart::Length(said),
        }
    }

    fn around(markup: &str, stretch: &TypedStretch) -> Vec<DraftPart> {
        let indexes = stretch.index_range();
        let said = Spelling::new(markup.get(indexes.clone()).unwrap_or_default().to_owned());
        let typed = Self::typed(stretch.kind(), said);
        Self::words(markup.get(..indexes.start).unwrap_or_default())
            .into_iter()
            .chain([typed])
            .chain(Self::split(markup.get(indexes.end..).unwrap_or_default()))
            .collect()
    }

    pub(super) fn split(markup: &str) -> Vec<DraftPart> {
        markup.parse::<TypedStretch>().map_or_else(
            |_| Self::words(markup).into_iter().collect(),
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
