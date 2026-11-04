use std::collections::{BTreeSet, HashMap};

use crate::reading::Reading;
use crate::ruby_text::{Letter, LetterCount, LetterIndex, RubyText};
use crate::words::Spelling;
use newtype::Counter;

#[derive(Debug, Default)]
pub struct NameSet {
    whole: BTreeSet<Spelling>,
    by_letter: HashMap<Letter, Vec<Vec<Letter>>>,
}

impl NameSet {
    #[must_use]
    #[expect(
        clippy::needless_for_each,
        reason = "iterators are driven by combinators here"
    )]
    pub fn new(names: Vec<Spelling>) -> NameSet {
        let whole: BTreeSet<Spelling> = names.into_iter().collect();
        let mut by_letter: HashMap<Letter, Vec<Vec<Letter>>> = HashMap::new();
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

    #[must_use]
    pub fn length_at(&self, text: &RubyText, at: LetterIndex) -> Option<LetterCount> {
        self.length_in(text, &text.letters(), at)
    }

    #[must_use]
    pub fn found_in(&self, text: &RubyText) -> Option<Spelling> {
        let written = text.letters();
        (0..written.len()).map(LetterIndex::new).find_map(|at| {
            self.length_in(text, &written, at)
                .map(|length| text.slice(&(at..at + length)).text(Reading::Written))
        })
    }

    fn length_in(
        &self,
        text: &RubyText,
        written: &[Letter],
        at: LetterIndex,
    ) -> Option<LetterCount> {
        self.by_letter
            .get(written.get(at.count())?)?
            .iter()
            .find_map(|name| Self::sits_at(text, written, name, at))
    }

    fn sits_at(
        text: &RubyText,
        written: &[Letter],
        name: &[Letter],
        at: LetterIndex,
    ) -> Option<LetterCount> {
        text.spelled_at(written, name, at)
            .filter(|span| {
                Self::glued(
                    span.start
                        .count()
                        .checked_sub(1)
                        .and_then(|before| written.get(before)),
                    name.first(),
                )
                .is_none()
            })
            .filter(|span| Self::glued(written.get(span.end.count()), name.last()).is_none())
            .map(|_| LetterCount::new(name.len()))
    }

    fn glued(neighbour: Option<&Letter>, edge: Option<&Letter>) -> Option<Letter> {
        neighbour
            .copied()
            .and_then(Letter::in_word)
            .and(edge.copied().and_then(Letter::in_word))
    }
}
