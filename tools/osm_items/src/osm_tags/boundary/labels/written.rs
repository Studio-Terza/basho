use std::str::FromStr;
use std::sync::LazyLock;

use jpreprocess::kind::JPreprocessDictionaryKind;
use jpreprocess::{DefaultTokenizer, JPreprocess, SystemDictionaryConfig};

use super::kana_reading::KanaReading;

static ANALYZER: LazyLock<JPreprocess<DefaultTokenizer>> = LazyLock::new(|| {
    let dictionary = SystemDictionaryConfig::Bundled(JPreprocessDictionaryKind::NaistJdic)
        .load()
        .expect("the bundled dictionary loads");
    JPreprocess::with_dictionaries(dictionary, None)
});

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Written(String);

impl Written {
    const HAN: std::ops::RangeInclusive<char> = '\u{3400}'..='\u{9fff}';

    pub fn reading(&self) -> Option<KanaReading> {
        let nodes = ANALYZER.text_to_njd(&self.0).ok()?.nodes;
        let parts = nodes
            .iter()
            .map(|node| Self::node_reading(node.get_read(), node.get_string()))
            .collect::<Option<Vec<KanaReading>>>()?;
        KanaReading::joined(parts)
    }

    fn node_reading(read: Option<&str>, written: &str) -> Option<KanaReading> {
        read.and_then(|read| read.parse::<KanaReading>().ok())
            .or_else(|| {
                written
                    .parse::<Written>()
                    .is_err()
                    .then(|| KanaReading::softened(written))
            })
    }
}

impl FromStr for Written {
    type Err = ();

    fn from_str(text: &str) -> Result<Written, ()> {
        if text.contains(|letter| Self::HAN.contains(&letter)) {
            Ok(Written(text.to_owned()))
        } else {
            Err(())
        }
    }
}
