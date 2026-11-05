mod note_kind;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

use crate::content::Content;
use crate::evidence::Evidence;
use crate::ruby_text::Filling;
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
        Content::deserialize(input).and_then(Self::filled::<Input::Error>)
    }

    fn filled<Failure: Error>(text: Content) -> Result<Content, Failure> {
        match text.filling() {
            Filling::Written => Ok(text),
            Filling::Blank => Err(Failure::custom("a fact needs its text")),
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
