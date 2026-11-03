mod claim;
mod run;
mod scan;

use std::collections::{BTreeMap, HashMap};

use guide::content::Content;
use guide::reading::Reading;
use guide::ruby_text::RubyText;
use guide::words::Spelling;

use crate::EntryId;
use crate::entry::{Entry, Section};
pub use claim::Claim;
use run::Run;
use scan::Scan;

#[derive(Debug, Default)]
pub struct NameTable {
    by_name: HashMap<Spelling, EntryId>,
}

impl NameTable {
    #[must_use]
    pub fn new(claims: &BTreeMap<Spelling, Claim>) -> NameTable {
        NameTable {
            by_name: claims
                .iter()
                .filter_map(|(name, claim)| claim.entry().map(|entry| (name.clone(), entry)))
                .collect(),
        }
    }

    #[must_use]
    pub fn entry_named(&self, name: &RubyText) -> Option<EntryId> {
        self.by_name.get(&name.text(Reading::Written)).copied()
    }

    #[must_use]
    pub fn runs(&self, content: &Content, shown: &RubyText, reader: Option<EntryId>) -> Vec<Run> {
        Scan::new(self, content, shown, reader).runs()
    }

    #[must_use]
    pub fn links_of(&self, entry: &Entry) -> Vec<EntryId> {
        let rows = entry
            .sections()
            .iter()
            .filter(|section| section.heading().mentioning().is_some())
            .flat_map(Section::rows)
            .flat_map(|row| {
                row.links().into_iter().chain(
                    row.texts()
                        .into_iter()
                        .flat_map(Content::mentions)
                        .filter_map(|name| self.entry_named(name)),
                )
            });
        let tagline = entry
            .tagline()
            .into_iter()
            .flat_map(Content::mentions)
            .filter_map(|name| self.entry_named(name));
        rows.chain(tagline).collect()
    }
}
