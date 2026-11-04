use crate::ruby_text::{LetterCount, LetterIndex};
use crate::words::Spelling;
use newtype::Counter;

impl LetterIndex {
    #[must_use]
    pub fn between(self, end: LetterIndex, text: &Spelling) -> Spelling {
        Spelling::new(
            text.chars()
                .skip(self.count())
                .take(end.count().saturating_sub(self.count()))
                .collect(),
        )
    }
}

impl LetterCount {
    #[must_use]
    pub fn letters_in(text: &Spelling) -> LetterCount {
        LetterCount::new(text.chars().count())
    }
}
