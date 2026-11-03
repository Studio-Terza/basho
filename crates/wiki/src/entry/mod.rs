mod entry_kind;
mod heading;
mod loop_row;
mod opening;
mod row;
mod section;
mod stop_row;

use guide::content::Content;
use guide::english_name::EnglishName;
use guide::evidence::Evidence;
use guide::ruby_text::RubyText;

use crate::Home;

pub use entry_kind::EntryKind;
pub use heading::Heading;
pub use loop_row::LoopRow;
pub use opening::Opening;
pub use row::{Row, SourcedFact};
pub use section::Section;
pub use stop_row::StopRow;

#[derive(Debug)]
pub struct Entry {
    kind: EntryKind,
    name: RubyText,
    english: Option<EnglishName>,
    tagline: Option<Content>,
    home: Option<Home>,
    evidence: Option<Evidence>,
    sections: Vec<Section>,
}

impl Entry {
    #[must_use]
    pub fn new(kind: EntryKind, name: RubyText) -> Entry {
        Entry {
            kind,
            name,
            english: None,
            tagline: None,
            home: None,
            evidence: None,
            sections: Vec::new(),
        }
    }

    #[must_use]
    pub fn titled(self, english: Option<EnglishName>, tagline: Option<Content>) -> Entry {
        Entry {
            english,
            tagline,
            ..self
        }
    }

    #[must_use]
    pub fn homed(self, home: Option<Home>) -> Entry {
        Entry { home, ..self }
    }

    #[must_use]
    pub fn backed_by(self, evidence: &Evidence) -> Entry {
        Entry {
            evidence: Some(evidence.clone()),
            ..self
        }
    }

    #[must_use]
    pub fn with_sections(self, sections: Vec<Section>) -> Entry {
        Entry {
            sections: sections
                .into_iter()
                .filter(|section| !section.rows().is_empty())
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn with_appended(self, section: Option<Section>) -> Entry {
        Entry {
            sections: self.sections.into_iter().chain(section).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn kind(&self) -> EntryKind {
        self.kind
    }

    #[must_use]
    pub fn name(&self) -> &RubyText {
        &self.name
    }

    #[must_use]
    pub fn english(&self) -> Option<&EnglishName> {
        self.english.as_ref()
    }

    #[must_use]
    pub fn tagline(&self) -> Option<&Content> {
        self.tagline.as_ref()
    }

    #[must_use]
    pub fn home(&self) -> Option<Home> {
        self.home
    }

    #[must_use]
    pub fn evidence(&self) -> Option<&Evidence> {
        self.evidence.as_ref()
    }

    #[must_use]
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }
}
