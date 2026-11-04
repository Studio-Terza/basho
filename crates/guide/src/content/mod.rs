mod content_error;
mod distance_unit;
pub mod figure;
mod name_kind;
mod part;
mod said_clock;
pub mod typed_part;
mod words;
pub use distance_unit::DistanceUnit;

use std::fmt;

use serde::Deserialize;

use crate::reading::Reading;
use crate::ruby_text::RubyText;

use content_error::ContentError;
use figure::Figure;
pub use name_kind::NameKind;
pub use part::Part;
pub use said_clock::SaidClock;
pub use typed_part::TypedPart;
use words::Words;
pub use words::{StretchKind, TypedStretch};

#[derive(Clone, Debug, Deserialize, derive_more::Deref)]
#[serde(from = "Vec<Part>")]
pub struct Content {
    parts: Vec<Part>,
    #[deref]
    shown: RubyText,
}

impl Content {
    #[must_use]
    pub fn parts(&self) -> &[Part] {
        &self.parts
    }

    #[must_use]
    pub fn shown(&self) -> &RubyText {
        &self.shown
    }

    pub fn mentions(&self) -> impl Iterator<Item = &RubyText> {
        self.parts
            .iter()
            .filter_map(|part| part.named(NameKind::Entry))
    }
}

impl From<Vec<Part>> for Content {
    fn from(parts: Vec<Part>) -> Content {
        let shown = RubyText::joined(parts.iter().map(Part::shown));
        Content { parts, shown }
    }
}

impl fmt::Display for Content {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.shown.text(Reading::Written))
    }
}
