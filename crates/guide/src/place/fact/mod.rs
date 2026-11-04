mod note_kind;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

use crate::content::Content;
use crate::evidence::Evidence;
use crate::trip::ContentList;

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
        text.letters()
            .iter()
            .all(|letter| letter.is_whitespace())
            .then(|| Input::Error::custom("a fact needs its text"))
            .map_or(Ok(text), Err)
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
