mod kana_reading;

use crate::catch_words::{FeatureName, Readings};
use std::collections::BTreeMap;

use serde::Deserialize;
use serde::de::IgnoredAny;

use kana_reading::KanaReading;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KanaFile {
    #[expect(dead_code, reason = "import list, kept only so the file loads")]
    unsourced: Option<IgnoredAny>,
    readings: BTreeMap<FeatureName, KanaReading>,
}

impl KanaFile {
    #[must_use]
    pub fn sourced(self) -> Readings {
        Readings::new(
            self.readings
                .into_iter()
                .filter_map(|(name, reading)| reading.sourced_kana().map(|kana| (name, kana)))
                .collect(),
        )
    }
}
