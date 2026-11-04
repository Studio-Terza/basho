mod condition_clause;
mod link_free;
mod markup;
mod object_phrase;
mod rewording;
mod uncited;
use std::fmt;

use condition_clause::ConditionClause;
use guide::ruby_text::RubyText;
use link_free::LinkFree;
use markup::Markup;
use newtype::Newtype;
use object_phrase::ObjectPhrase;
pub use rewording::Rewording;
use uncited::Uncited;

pub struct Wording<'text> {
    original: &'text RubyText,
    markup: Markup,
}

impl<'text> Wording<'text> {
    #[must_use]
    pub fn new(original: &'text RubyText) -> Wording<'text> {
        let markup = Markup::new(original.markup().to_string());
        Wording { original, markup }
    }

    #[must_use]
    pub fn reworded(self, rewording: Rewording) -> Wording<'text> {
        let markup = Self::applied(rewording, &self.markup);
        Wording { markup, ..self }
    }

    fn applied(rewording: Rewording, markup: &Markup) -> Markup {
        match rewording {
            Rewording::ForWalker => Markup::new(
                Uncited::new(&Markup::new(LinkFree::new(markup).to_string())).to_string(),
            ),
            Rewording::ConditionClause => Markup::new(ConditionClause::new(markup).to_string()),
            Rewording::ObjectPhrase => Markup::new(ObjectPhrase::new(markup).to_string()),
        }
    }

    #[must_use]
    pub fn ruby(self) -> RubyText {
        RubyText::try_from(self.markup.into_inner()).unwrap_or_else(|_| self.original.clone())
    }
}

impl fmt::Display for Wording<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.markup)
    }
}
