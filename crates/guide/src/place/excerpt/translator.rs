use crate::words::Label;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Translator {
    Ours,
    WrittenInEnglish,
}

impl Translator {
    #[must_use]
    pub fn label(self) -> Label {
        match self {
            Translator::Ours => Label::new("Our translation"),
            Translator::WrittenInEnglish => Label::new("Written in English; no translation"),
        }
    }
}
