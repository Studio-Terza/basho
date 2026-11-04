use std::fmt;

use guide::content::Content;
use guide::place::Fact;
use guide::words::Spelling;

use super::ruby_markup::RubyMarkup;

pub struct FactMarkup<'fact>(&'fact Fact);

impl<'fact> FactMarkup<'fact> {
    #[must_use]
    pub fn new(fact: &'fact Fact) -> FactMarkup<'fact> {
        FactMarkup(fact)
    }
}

impl fmt::Display for FactMarkup<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ", RubyMarkup::new(self.0.text()))?;
        self.verdict(formatter)
    }
}

impl FactMarkup<'_> {
    fn verdict(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.evidence().backing() {
            Some(source) => Self::checked(formatter, &self.word(), source.checked()),
            None => Self::unverified(formatter, &self.word(), self.0.evidence().reason()),
        }
    }

    fn word(&self) -> Spelling {
        self.0.evidence().kind().word().lowered()
    }

    fn checked(
        formatter: &mut fmt::Formatter<'_>,
        word: &Spelling,
        checked: impl fmt::Display,
    ) -> fmt::Result {
        write!(formatter, "<span class=\"checked\">{word} {checked}</span>")
    }

    fn unverified(
        formatter: &mut fmt::Formatter<'_>,
        word: &Spelling,
        reason: Option<&Content>,
    ) -> fmt::Result {
        write!(formatter, "<span class=\"unverified\">{word}: ")?;
        reason.map_or(Ok(()), |reason| {
            write!(formatter, "{}", RubyMarkup::new(reason))
        })?;
        formatter.write_str("</span>")
    }
}
