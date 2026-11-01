use crate::task::trip_text::SectionText;
use serde::Serialize;

use super::day_file::DayFile;

#[derive(Clone, Debug, Serialize)]
pub struct CatchSection {
    kana: DayFile,
    catches: Vec<DayFile>,
}

impl CatchSection {
    pub fn new(kana: DayFile, catches: Vec<DayFile>) -> CatchSection {
        CatchSection {
            kana: kana.without_nulls(),
            catches: catches.into_iter().map(DayFile::without_nulls).collect(),
        }
    }

    pub fn text(&self) -> Result<SectionText, toml::ser::Error> {
        toml::to_string(self).map(SectionText::new)
    }
}
