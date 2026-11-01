use std::collections::{BTreeSet, HashMap};

use crate::place::RubyText;
use crate::reading::Reading;
use crate::words::Spelling;

#[derive(Debug, Default)]
pub struct NameSet {
    whole: BTreeSet<Spelling>,
    by_letter: HashMap<char, Vec<Vec<char>>>,
}

impl NameSet {
    #[must_use]
    #[expect(
        clippy::needless_for_each,
        reason = "iterators are driven by combinators here"
    )]
    pub fn new(names: Vec<Spelling>) -> NameSet {
        let whole: BTreeSet<Spelling> = names.into_iter().collect();
        let mut by_letter: HashMap<char, Vec<Vec<char>>> = HashMap::new();
        whole
            .iter()
            .map(|name| RubyText::verbatim(name.clone()).letters())
            .for_each(|letters| {
                by_letter
                    .entry(letters.first().copied().unwrap_or_default())
                    .or_default()
                    .push(letters);
            });
        by_letter
            .values_mut()
            .for_each(|names| names.sort_by_key(|name| std::cmp::Reverse(name.len())));
        NameSet { whole, by_letter }
    }

    #[must_use]
    pub fn holds(&self, name: &Spelling) -> bool {
        self.whole.contains(name)
    }

    fn length_in(&self, text: &RubyText, written: &[char], at_index: usize) -> Option<usize> {
        self.by_letter
            .get(written.get(at_index)?)?
            .iter()
            .find(|name| Self::sits_at(text, written, name, at_index))
            .map(Vec::len)
    }

    #[must_use]
    pub fn length_at(&self, text: &RubyText, at_index: usize) -> Option<usize> {
        self.length_in(text, &text.letters(), at_index)
    }

    #[must_use]
    pub fn found_in(&self, text: &RubyText) -> Option<Spelling> {
        let written = text.letters();
        (0..written.len()).find_map(|at_index| {
            self.length_in(text, &written, at_index).map(|length| {
                text.slice(at_index, at_index + length)
                    .text(Reading::Written)
            })
        })
    }

    fn sits_at(text: &RubyText, written: &[char], name: &[char], at_index: usize) -> bool {
        let end_index = at_index + name.len();
        written.get(at_index..end_index) == Some(name)
            && text.parts_at(at_index)
            && text.parts_at(end_index)
            && Self::clear(
                at_index
                    .checked_sub(1)
                    .and_then(|before| written.get(before)),
                name.first(),
            )
            && Self::clear(written.get(end_index), name.last())
    }

    fn clear(neighbour: Option<&char>, edge: Option<&char>) -> bool {
        !(neighbour.is_some_and(char::is_ascii_alphanumeric)
            && edge.is_some_and(char::is_ascii_alphanumeric))
    }
}
