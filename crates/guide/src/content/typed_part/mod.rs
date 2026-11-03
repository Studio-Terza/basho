mod measure;
mod price;
mod scheduled_train;
pub use measure::{Measure, SpanUnit};
pub use price::Price;
pub use scheduled_train::ScheduledTrain;
use std::fmt;

use crate::content::DistanceUnit;

use crate::FactSource;
use serde::Deserialize;

use crate::content::{NameKind, SaidClock};
use crate::evidence::EvidenceKind;
use crate::reading::Reading;
use crate::ruby_text::RubyText;
use crate::words::{Prose, Spelling};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum TypedPart {
    #[serde(rename = "plan")]
    Intended(SaidClock),
    #[serde(rename = "fact")]
    Stated(SaidFact),
    #[serde(rename = "unknown")]
    Unsure(SaidClock),
    #[serde(rename = "entry")]
    Mention(RubyText),
    #[serde(rename = "place")]
    Spot(RubyText),
    #[serde(rename = "distance")]
    Length(Measure<DistanceUnit>),
    Span(Measure<SpanUnit>),
    #[serde(rename = "price")]
    Cost(Price),
    #[serde(rename = "train")]
    Service(ScheduledTrain),
    Quote(Prose),
}

impl TypedPart {
    #[must_use]
    pub fn shown(&self) -> RubyText {
        match self {
            TypedPart::Mention(name) | TypedPart::Spot(name) => name.clone(),
            _ => RubyText::verbatim(Spelling::new(self.to_string())),
        }
    }

    #[must_use]
    pub fn named(&self, kind: NameKind) -> Option<&RubyText> {
        match (self, kind) {
            (TypedPart::Mention(name), NameKind::Entry)
            | (TypedPart::Spot(name), NameKind::Spot) => Some(name),
            _ => None,
        }
    }
}

impl fmt::Display for TypedPart {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypedPart::Intended(clock) => clock.fmt(formatter),
            TypedPart::Stated(fact) => fact.clock().fmt(formatter),
            TypedPart::Unsure(clock) => match EvidenceKind::Unverified.mark() {
                Some(mark) => write!(formatter, "{clock}{mark}"),
                None => clock.fmt(formatter),
            },
            TypedPart::Mention(name) | TypedPart::Spot(name) => {
                formatter.write_str(&name.text(Reading::Written))
            }
            TypedPart::Length(distance) => distance.fmt(formatter),
            TypedPart::Span(span) => span.fmt(formatter),
            TypedPart::Cost(price) => price.fmt(formatter),
            TypedPart::Service(train) => train.fmt(formatter),
            TypedPart::Quote(quote) => quote.fmt(formatter),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaidFact {
    at: SaidClock,
    source: FactSource,
    #[serde(deserialize_with = "EvidenceKind::deserialize_backed")]
    evidence: Option<EvidenceKind>,
}

impl SaidFact {
    #[must_use]
    pub fn clock(self) -> SaidClock {
        self.at
    }

    #[must_use]
    pub fn source(self) -> FactSource {
        self.source
    }

    #[must_use]
    pub fn evidence(self) -> Option<EvidenceKind> {
        self.evidence
    }
}
