use crate::catch_words::FeatureName;
use crate::words::Spelling;
use serde::Deserialize;
use serde::de::IgnoredAny;
use std::cmp::Ordering;

#[derive(Debug, Default, Deserialize)]
#[serde(from = "Option<Spelling>")]
enum Citation {
    #[default]
    Uncited,
    Cited,
}

impl From<Option<Spelling>> for Citation {
    fn from(source: Option<Spelling>) -> Citation {
        match source.map(|spelling| spelling.cmp(&Spelling::default())) {
            Some(Ordering::Greater) => Citation::Cited,
            Some(Ordering::Less | Ordering::Equal) | None => Citation::Uncited,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KanaReading {
    kana: FeatureName,
    #[serde(default)]
    source: Citation,
    #[expect(dead_code, reason = "import date, kept only so the file loads")]
    checked: Option<IgnoredAny>,
    #[expect(dead_code, reason = "import note, kept only so the file loads")]
    note: Option<IgnoredAny>,
}

impl KanaReading {
    #[must_use]
    pub fn sourced_kana(self) -> Option<FeatureName> {
        match self.source {
            Citation::Cited => Some(self.kana),
            Citation::Uncited => None,
        }
    }
}
