use newtype::{Arbitrary, Counter, Newtype};
use std::fmt;
use std::ops::{Add, Range};

#[derive(Newtype, Arbitrary, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LetterIndex(usize);

#[derive(Newtype, Arbitrary, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LetterCount(usize);

#[derive(Newtype, Arbitrary, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Letter(char);

impl Add<LetterCount> for LetterIndex {
    type Output = LetterIndex;

    fn add(self, count: LetterCount) -> LetterIndex {
        LetterIndex::new(self.count().saturating_add(count.count()))
    }
}

impl LetterIndex {
    #[must_use]
    pub fn inside(self, span: &Range<LetterIndex>) -> Option<LetterIndex> {
        Some(self).filter(|at| (span.start.incremented()..span.end).contains(at))
    }

    pub const END: LetterIndex = LetterIndex::new(usize::MAX);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Writing {
    Japanese,
    Latin,
}

impl Letter {
    const WORD_LETTERS: [std::ops::RangeInclusive<char>; 3] = ['0'..='9', 'A'..='Z', 'a'..='z'];

    #[must_use]
    pub fn writing(self) -> Writing {
        Some(self)
            .filter(|letter| ('\u{3040}'..='\u{9FFF}').contains(&letter.0))
            .map_or(Writing::Latin, |_| Writing::Japanese)
    }

    #[must_use]
    pub fn in_word(self) -> Option<Letter> {
        Self::WORD_LETTERS
            .iter()
            .find(|letters| letters.contains(&self.0))
            .map(|_| self)
    }
}

impl From<char> for Letter {
    fn from(letter: char) -> Letter {
        Letter(letter)
    }
}

impl fmt::Display for LetterCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
