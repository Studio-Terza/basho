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
        Label::new(match self {
            Translator::Ours => "Our translation",
            Translator::WrittenInEnglish => "Written in English; no translation",
        })
    }
}
