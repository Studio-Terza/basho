use crate::words::{Label, Spelling};
use serde::Deserialize;

#[derive(newtype::Display, Clone, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct Url(String);

impl Url {
    const PLANS: &'static str = "planning/";

    #[must_use]
    pub fn is_planning(&self) -> bool {
        let file = self.0.split('#').next().unwrap_or(&self.0);
        file.starts_with(Self::PLANS)
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
        if text.trim().is_empty() {
            Err(Label::new("a source needs an address"))
        } else {
            Ok(Url(text))
        }
    }
}
