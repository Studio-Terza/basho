mod note_kind;
use serde::Deserialize;

use crate::content::{Content, ContentList};
use crate::evidence::Evidence;

pub use note_kind::NoteKind;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    text: Content,
    #[serde(default)]
    kind: NoteKind,
    evidence: Evidence,
}

impl Fact {
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
