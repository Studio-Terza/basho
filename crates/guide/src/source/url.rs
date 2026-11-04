use crate::words::{Label, Spelling};
use serde::Deserialize;

#[derive(newtype::Display, Clone, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct Url(String);

impl Url {
    const PLANS: &'static str = "planning/";

    #[must_use]
    pub fn is_planning(&self) -> Option<&Url> {
        Some(self).filter(|url| {
            url.0
                .split('#')
                .next()
                .unwrap_or(&url.0)
                .starts_with(Self::PLANS)
        })
    }

    #[must_use]
    pub fn host(&self) -> Spelling {
        let rest = self
            .0
            .split_once("://")
            .map_or(self.0.as_str(), |(_, rest)| rest);
        Spelling::new(
            rest.split(['/', '?', '#'])
                .next()
                .unwrap_or(rest)
                .to_owned(),
        )
    }
}

impl TryFrom<String> for Url {
    type Error = Label;

    fn try_from(text: String) -> Result<Url, Label> {
        text.trim()
            .chars()
            .next()
            .map(|_| Url(text))
            .ok_or_else(|| Label::new("a source needs an address"))
    }
}
