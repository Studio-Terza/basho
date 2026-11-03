use serde::Deserialize;

use super::train_text_error::TrainTextError;
use crate::reading::Reading;
use crate::ruby_text::RubyText;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(try_from = "RubyText")]
pub struct Headsign(RubyText);

impl Headsign {
    const ENDINGS: [&str; 2] = ["行き", "方面"];
}

impl TryFrom<RubyText> for Headsign {
    type Error = TrainTextError;

    fn try_from(text: RubyText) -> Result<Headsign, TrainTextError> {
        let written = text.text(Reading::Written);
        Self::ENDINGS
            .iter()
            .filter_map(|ending| written.strip_suffix(ending))
            .any(|destination| !destination.is_empty())
            .then_some(Headsign(text))
            .ok_or(TrainTextError::Headsign(written))
    }
}
