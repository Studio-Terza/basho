use serde::Deserialize;

use super::train_text_error::TrainTextError;
use crate::reading::Reading;
use crate::ruby_text::RubyText;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(try_from = "RubyText")]
pub struct LineName(RubyText);

impl TryFrom<RubyText> for LineName {
    type Error = TrainTextError;

    fn try_from(text: RubyText) -> Result<LineName, TrainTextError> {
        text.text(Reading::Written)
            .trim()
            .chars()
            .next()
            .map(|_| LineName(text))
            .ok_or(TrainTextError::NamelessLine)
    }
}
