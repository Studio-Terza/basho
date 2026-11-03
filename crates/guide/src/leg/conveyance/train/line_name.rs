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
        if text.text(Reading::Written).trim().is_empty() {
            Err(TrainTextError::NamelessLine)
        } else {
            Ok(LineName(text))
        }
    }
}
