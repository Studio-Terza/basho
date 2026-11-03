use std::fmt;

use serde::Deserialize;

use crate::clock::{ClockTime, TripTime};
use crate::evidence::{Evidence, EvidenceKind};
use crate::fact_source::FactSource;
use crate::times::clock_digits::ClockDigits;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FactTime {
    fact: ClockTime,
    source: FactSource,
    #[serde(deserialize_with = "EvidenceKind::deserialize_backed")]
    evidence: Option<EvidenceKind>,
}

impl FactTime {
    #[must_use]
    pub fn new(fact: ClockTime, source: FactSource, evidence: &Evidence) -> FactTime {
        FactTime {
            fact,
            source,
            evidence: Some(evidence.kind()),
        }
    }

    #[must_use]
    pub fn from_gps(fix: TripTime) -> FactTime {
        FactTime {
            fact: fix.time(),
            source: FactSource::Gps,
            evidence: None,
        }
    }

    #[must_use]
    pub fn at_sunset(fact: ClockTime) -> FactTime {
        FactTime {
            fact,
            source: FactSource::AtSunset,
            evidence: None,
        }
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

impl From<FactTime> for ClockTime {
    fn from(time: FactTime) -> ClockTime {
        time.fact
    }
}

impl fmt::Display for FactTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.source.beside(ClockDigits::new(self.fact), formatter)?;
        self.evidence
            .and_then(EvidenceKind::mark)
            .map_or(Ok(()), |mark| write!(formatter, " {mark}"))
    }
}
