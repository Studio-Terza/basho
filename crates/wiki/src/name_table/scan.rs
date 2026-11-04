use guide::content::Content;
use guide::ruby_text::{Letter, LetterIndex, RubyText};
use newtype::Counter;
use std::ops::Range;

use crate::EntryId;
use crate::name_table::{NameTable, Run};

struct Mentioned {
    span: Range<LetterIndex>,
    entry: Option<EntryId>,
}

pub struct Scan<'scan> {
    table: &'scan NameTable,
    content: &'scan Content,
    shown: &'scan RubyText,
    written: Vec<Letter>,
    reader: Option<EntryId>,
}

impl<'scan> Scan<'scan> {
    #[must_use]
    pub fn new(
        table: &'scan NameTable,
        content: &'scan Content,
        shown: &'scan RubyText,
        reader: Option<EntryId>,
    ) -> Scan<'scan> {
        Scan {
            table,
            content,
            shown,
            written: shown.letters(),
            reader,
        }
    }

    #[must_use]
    pub fn runs(&self) -> Vec<Run> {
        let found: Vec<Mentioned> = self
            .content
            .mentions()
            .fold(Vec::new(), |found, name| self.with_mention(found, name))
            .into_iter()
            .filter(|mention| mention.entry.is_some())
            .collect();
        let last_index = found
            .last()
            .map_or(LetterIndex::default(), |mention| mention.span.end);
        let gaps = found
            .iter()
            .scan(LetterIndex::default(), |end_index, mention| {
                let start_index = std::mem::replace(end_index, mention.span.end);
                Some(Run::new(
                    self.shown.slice(&(start_index..mention.span.start)),
                    None,
                ))
            });
        gaps.zip(found.iter())
            .flat_map(|(gap, mention)| {
                [
                    gap,
                    Run::new(self.shown.slice(&mention.span), mention.entry),
                ]
            })
            .chain([Run::new(
                self.shown
                    .slice(&(last_index..LetterIndex::new(self.written.len()))),
                None,
            )])
            .filter(|run| !run.text().segments().is_empty())
            .collect()
    }

    fn mention_at(
        &self,
        name: &RubyText,
        letters: &[Letter],
        at: LetterIndex,
    ) -> Option<Mentioned> {
        self.shown
            .spelled_at(&self.written, letters, at)
            .map(|span| Mentioned {
                span,
                entry: self
                    .table
                    .entry_named(name)
                    .filter(|entry| Some(*entry) != self.reader),
            })
    }

    fn with_mention(&self, mut found: Vec<Mentioned>, name: &RubyText) -> Vec<Mentioned> {
        let letters = name.letters();
        let from_index = found
            .last()
            .map_or(LetterIndex::default(), |mention| mention.span.end);
        found.extend(
            (from_index.count()..self.written.len())
                .map(LetterIndex::new)
                .find_map(|start_index| self.mention_at(name, &letters, start_index)),
        );
        found
    }
}
