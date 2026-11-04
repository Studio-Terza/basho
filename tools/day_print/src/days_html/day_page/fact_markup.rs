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
        let marked = evidence.backing().map_or_else(
            || {
                format!(
                    "<span class=\"unverified\">{word}: {}</span>",
                    evidence
                        .reason()
                        .map_or(String::new(), |reason| RubyMarkup::new(reason).to_string())
                )
            },
            |source| format!("<span class=\"checked\">{word} {}</span>", source.checked()),
        );
        formatter.write_str(&marked)
    }
}
