use serde::Deserialize;

use crate::content::SaidClock;
use crate::evidence::EvidenceKind;
use crate::times::FactSource;

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
