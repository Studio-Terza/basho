use std::fmt;

use serde::Deserialize;

use crate::content::{
    DistanceUnit, Measure, NameKind, Price, SaidClock, SaidFact, ScheduledTrain, SpanUnit,
};
use crate::evidence::EvidenceKind;
use crate::place::RubyText;
use crate::reading::Reading;
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
