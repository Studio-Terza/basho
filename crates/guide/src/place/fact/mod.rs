mod note_kind;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

use crate::content::{Content, ContentList};
use crate::evidence::Evidence;

pub use note_kind::NoteKind;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    #[serde(deserialize_with = "Fact::stated")]
    text: Content,
    #[serde(default)]
    kind: NoteKind,
    evidence: Evidence,
}

impl Fact {
    fn stated<'de, Input: Deserializer<'de>>(input: Input) -> Result<Content, Input::Error> {
        let text = Content::deserialize(input)?;
        if text.letters().iter().all(|letter| letter.is_whitespace()) {
            Err(Input::Error::custom("a fact needs its text"))
        } else {
            Ok(text)
        }
    }

    #[must_use]
    pub fn kind(&self) -> NoteKind {
        self.kind
    }

    #[must_use]
    pub fn evidence(&self) -> &Evidence {
        &self.evidence
    }
}

impl Fact {
    #[must_use]
    pub fn text(&self) -> &Content {
        &self.text
    }
}

impl<'fact> From<&'fact Fact> for ContentList<'fact> {
    fn from(fact: &'fact Fact) -> ContentList<'fact> {
        std::iter::once(fact.text())
            .chain(fact.evidence().reason())
            .collect()
    }
}
