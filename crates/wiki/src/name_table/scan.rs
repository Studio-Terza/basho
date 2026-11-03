use guide::content::Content;
use guide::ruby_text::RubyText;

use crate::EntryId;
use crate::name_table::{NameTable, Run};

struct Mentioned {
    start_index: usize,
    end_index: usize,
    entry: Option<EntryId>,
}

pub struct Scan<'scan> {
    table: &'scan NameTable,
    content: &'scan Content,
    shown: &'scan RubyText,
    written: Vec<char>,
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
        let last_index = found.last().map_or(0, |mention| mention.end_index);
        let gaps = found.iter().scan(0, |end_index, mention| {
            let start_index = std::mem::replace(end_index, mention.end_index);
            Some(Run::new(
                self.shown.slice(start_index, mention.start_index),
                None,
            ))
        });
        gaps.zip(found.iter())
            .flat_map(|(gap, mention)| {
                [
                    gap,
                    Run::new(
                        self.shown.slice(mention.start_index, mention.end_index),
                        mention.entry,
                    ),
                ]
            })
            .chain([Run::new(
                self.shown.slice(last_index, self.written.len()),
                None,
            )])
            .filter(|run| !run.text().segments().is_empty())
            .collect()
    }

    fn mention_at(
        &self,
        name: &RubyText,
        letters: &[char],
        start_index: usize,
    ) -> Option<Mentioned> {
        let end_index = start_index + letters.len();
        (self.written.get(start_index..end_index) == Some(letters)
            && self.shown.parts_at(start_index)
            && self.shown.parts_at(end_index))
        .then(|| Mentioned {
            start_index,
            end_index,
            entry: self
                .table
                .entry_named(name)
                .filter(|entry| Some(*entry) != self.reader),
        })
    }

    fn with_mention(&self, mut found: Vec<Mentioned>, name: &RubyText) -> Vec<Mentioned> {
        let letters = name.letters();
        let from_index = found.last().map_or(0, |mention| mention.end_index);
        found.extend(
            (from_index..self.written.len())
                .find_map(|start_index| self.mention_at(name, &letters, start_index)),
        );
        found
    }
}
