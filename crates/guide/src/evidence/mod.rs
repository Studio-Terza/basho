mod backing;
mod evidence_counts;
mod evidence_tally;
mod fact_count;
mod source;

use std::time::{SystemTime, UNIX_EPOCH};

use chrono::DateTime;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use strum::EnumDiscriminants;

use crate::clock::CalendarDate;
use crate::content::Content;
use crate::words::Label;

pub use backing::Backing;
pub use evidence_counts::EvidenceCounts;
pub use evidence_tally::EvidenceTally;
pub use fact_count::FactCount;
pub use source::Source;

#[derive(Clone, Debug, Deserialize, EnumDiscriminants)]
#[strum_discriminants(name(EvidenceKind), derive(PartialOrd, Ord))]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Evidence {
    Unverified(#[serde(deserialize_with = "Evidence::said")] Content),
    Source(#[serde(deserialize_with = "Evidence::checked")] Source),
    Simulator(#[serde(deserialize_with = "Evidence::checked")] Source),
    Owner(#[serde(deserialize_with = "Evidence::checked")] Source),
}

impl Evidence {
    fn said<'de, Input: Deserializer<'de>>(input: Input) -> Result<Content, Input::Error> {
        let reason = Content::deserialize(input)?;
        if reason.letters().iter().all(|letter| letter.is_whitespace()) {
            Err(Input::Error::custom("an unverified fact needs its reason"))
        } else {
            Ok(reason)
        }
    }

    fn checked<'de, Input: Deserializer<'de>>(input: Input) -> Result<Source, Input::Error> {
        let source = Source::deserialize(input)?;
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_secs());
        let today = i64::try_from(seconds)
            .ok()
            .and_then(|seconds| DateTime::from_timestamp(seconds, 0))
            .map(|now| now.date_naive());
        let latest = today
            .and_then(|today| today.succ_opt())
            .map(CalendarDate::from);
        if source.url().is_planning() {
            Err(Input::Error::custom(
                "the trip's own plan cannot back a fact: cite an outside source or mark it unverified",
            ))
        } else if latest.is_some_and(|latest| source.checked() > latest) {
            Err(Input::Error::custom(
                "a source cannot be checked in the future",
            ))
        } else {
            Ok(source)
        }
    }

    #[must_use]
    pub fn kind(&self) -> EvidenceKind {
        EvidenceKind::from(self)
    }

    #[must_use]
    pub fn backing(&self) -> Option<&Source> {
        match self {
            Evidence::Unverified(_) => None,
            Evidence::Source(source) | Evidence::Simulator(source) | Evidence::Owner(source) => {
                Some(source)
            }
        }
    }

    #[must_use]
    pub fn reason(&self) -> Option<&Content> {
        match self {
            Evidence::Unverified(reason) => Some(reason),
            Evidence::Source(_) | Evidence::Simulator(_) | Evidence::Owner(_) => None,
        }
    }

    #[must_use]
    pub fn mark(&self) -> Option<Label> {
        self.kind().mark()
    }
}

impl EvidenceKind {
    pub fn deserialize_backed<'de, Input: Deserializer<'de>>(
        input: Input,
    ) -> Result<Option<EvidenceKind>, Input::Error> {
        Evidence::deserialize(input).map(|evidence| Some(evidence.kind()))
    }

    #[must_use]
    pub fn word(self) -> Label {
        Label::new(match self {
            EvidenceKind::Unverified => "Unverified",
            EvidenceKind::Source => "Checked",
            EvidenceKind::Simulator => "Seen in simulator",
            EvidenceKind::Owner => "Confirmed in use",
        })
    }

    #[must_use]
    pub fn mark(self) -> Option<Label> {
        match self {
            EvidenceKind::Unverified => Some(Label::new("?")),
            EvidenceKind::Source | EvidenceKind::Simulator | EvidenceKind::Owner => None,
        }
    }
}
