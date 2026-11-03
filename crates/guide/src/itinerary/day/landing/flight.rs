use serde::Deserialize;

use crate::ruby_text::RubyText;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Flight(RubyText);

impl AsRef<RubyText> for Flight {
    fn as_ref(&self) -> &RubyText {
        &self.0
    }
}
