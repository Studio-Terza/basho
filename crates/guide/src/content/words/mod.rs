mod typed_stretch;

use serde::Deserialize;

use crate::content::ContentError;
use crate::reading::Reading;
use crate::ruby_text::RubyText;
use crate::words::Written;

pub use typed_stretch::{StretchKind, TypedStretch};

#[derive(Clone, Debug, Deserialize, derive_more::AsRef)]
#[serde(try_from = "String")]
pub struct Words(RubyText);

impl TryFrom<String> for Words {
    type Error = ContentError;

    fn try_from(text: String) -> Result<Words, ContentError> {
        let ruby = RubyText::try_from(text.clone())
            .map_err(|error| ContentError::Unread(Written::new(error.to_string())))?;
        match ruby
            .text(Reading::Written)
            .parse::<TypedStretch>()
            .as_ref()
            .map(TypedStretch::kind)
        {
            Ok(StretchKind::Clock) => Err(ContentError::ClockInWords(Written::new(text))),
            Ok(StretchKind::Distance) => Err(ContentError::DistanceInWords(Written::new(text))),
            Err(_) => Ok(Words(ruby)),
        }
    }
}
