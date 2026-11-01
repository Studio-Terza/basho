mod card_line;

use serde::Deserialize;

use crate::content::Content;
use crate::leg::card_fill::CardFill;

pub use card_line::CardLine;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Card {
    purpose: Content,
    japanese: Content,
    english: Content,
    fill: Option<CardFill>,
}

impl Card {
    #[must_use]
    pub fn line_text(&self, line: CardLine) -> &Content {
        match line {
            CardLine::Purpose => &self.purpose,
            CardLine::Japanese => &self.japanese,
            CardLine::English => &self.english,
        }
    }

    #[must_use]
    pub fn fill(&self) -> Option<CardFill> {
        self.fill
    }
}
