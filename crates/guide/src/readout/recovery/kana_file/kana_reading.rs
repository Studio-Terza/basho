use crate::readout::recovery::catch_words::FeatureName;
use crate::words::Spelling;
use serde::Deserialize;
use serde::de::IgnoredAny;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KanaReading {
    kana: FeatureName,
    source: Option<Spelling>,
    #[expect(dead_code, reason = "import date, kept only so the file loads")]
    checked: Option<IgnoredAny>,
    #[expect(dead_code, reason = "import note, kept only so the file loads")]
    note: Option<IgnoredAny>,
}

impl KanaReading {
    #[must_use]
    pub fn sourced_kana(self) -> Option<FeatureName> {
        self.source
            .filter(|source| !source.is_empty())
            .map(|_| self.kana)
    }
}
