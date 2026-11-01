use newtype::Newtype;
use std::fmt;

#[derive(Newtype)]
pub struct ObjectPhrase<'text> {
    markup: &'text str,
}

impl ObjectPhrase<'_> {
    const ARTICLES: [&'static str; 3] = ["The ", "A ", "An "];
}

impl fmt::Display for ObjectPhrase<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowered = Self::ARTICLES.iter().find_map(|article| {
            self.markup
                .strip_prefix(article)
                .map(|rest| format!("{}{rest}", article.to_lowercase()))
        });
        formatter.write_str(lowered.as_deref().unwrap_or(self.markup))
    }
}
