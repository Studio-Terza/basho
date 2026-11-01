mod condition_clause;
mod link_free;
mod object_phrase;
mod rewording;
mod uncited;
use std::fmt;

use condition_clause::ConditionClause;
use guide::place::RubyText;
use link_free::LinkFree;
use object_phrase::ObjectPhrase;
pub use rewording::Rewording;
use uncited::Uncited;

pub struct Wording<'text> {
    original: &'text RubyText,
    markup: String,
}

impl<'text> Wording<'text> {
    #[must_use]
    pub fn new(original: &'text RubyText) -> Wording<'text> {
        let markup = original.markup().to_string();
        Wording { original, markup }
    }

    #[must_use]
    pub fn reworded(self, rewording: Rewording) -> Wording<'text> {
        let markup = match rewording {
            Rewording::ForWalker => {
                Uncited::new(&LinkFree::new(&self.markup).to_string()).to_string()
            }
            Rewording::ConditionClause => ConditionClause::new(&self.markup).to_string(),
            Rewording::ObjectPhrase => ObjectPhrase::new(&self.markup).to_string(),
        };
        Wording { markup, ..self }
    }

    #[must_use]
    pub fn ruby(self) -> RubyText {
        RubyText::try_from(self.markup).unwrap_or_else(|_| self.original.clone())
    }
}

impl fmt::Display for Wording<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.markup)
    }
}
