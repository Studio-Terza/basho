use guide::place::RubyText;

use crate::EntryId;

#[derive(Clone, Debug)]
pub struct Run {
    text: RubyText,
    entry: Option<EntryId>,
}

impl Run {
    #[must_use]
    pub fn new(text: RubyText, entry: Option<EntryId>) -> Run {
        Run { text, entry }
    }

    #[must_use]
    pub fn text(&self) -> &RubyText {
        &self.text
    }

    #[must_use]
    pub fn entry(&self) -> Option<EntryId> {
        self.entry
    }
}
