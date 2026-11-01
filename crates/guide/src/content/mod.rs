mod content_error;
mod content_list;
mod figure;
mod measure;
mod name_kind;
mod part;
mod price;
mod said_clock;
mod said_fact;
mod scheduled_train;
mod typed_part;
mod words;

use std::fmt;

use serde::Deserialize;

use crate::place::RubyText;
use crate::reading::Reading;

use content_error::ContentError;
pub use content_list::ContentList;
use figure::Figure;
use measure::{DistanceUnit, Measure, SpanUnit};
pub use name_kind::NameKind;
pub use part::Part;
use price::Price;
use said_clock::SaidClock;
use said_fact::SaidFact;
use scheduled_train::ScheduledTrain;
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
