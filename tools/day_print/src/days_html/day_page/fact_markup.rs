use std::fmt;

use guide::place::Fact;

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
        let evidence = self.0.evidence();
        let word = evidence.kind().word().lowered();
        if let Some(source) = evidence.backing() {
            write!(
                formatter,
                "<span class=\"checked\">{word} {}</span>",
                source.checked()
            )
        } else {
            write!(formatter, "<span class=\"unverified\">{word}: ")?;
            evidence.reason().map_or(Ok(()), |reason| {
                write!(formatter, "{}", RubyMarkup::new(reason))
            })?;
            formatter.write_str("</span>")
        }
    }
}
