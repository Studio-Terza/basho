use std::fmt;
use std::iter::Sum;
use std::ops::Add;

use serde::Serialize;

use crate::evidence::{EvidenceKind, FactCount};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct EvidenceCounts {
    unverified: FactCount,
    source: FactCount,
    simulator: FactCount,
    owner: FactCount,
}

impl EvidenceCounts {
    #[must_use]
    pub fn total(self) -> FactCount {
        self.unverified + self.source + self.simulator + self.owner
    }
}

impl From<EvidenceKind> for EvidenceCounts {
    fn from(kind: EvidenceKind) -> EvidenceCounts {
        let one = FactCount::new(1);
        match kind {
            EvidenceKind::Unverified => EvidenceCounts {
                unverified: one,
                ..Self::default()
            },
            EvidenceKind::Source => EvidenceCounts {
                source: one,
                ..Self::default()
            },
            EvidenceKind::Simulator => EvidenceCounts {
                simulator: one,
                ..Self::default()
            },
            EvidenceKind::Owner => EvidenceCounts {
                owner: one,
                ..Self::default()
            },
        }
    }
}

impl Add for EvidenceCounts {
    type Output = EvidenceCounts;

    fn add(self, other: EvidenceCounts) -> EvidenceCounts {
        EvidenceCounts {
            unverified: self.unverified + other.unverified,
            source: self.source + other.source,
            simulator: self.simulator + other.simulator,
            owner: self.owner + other.owner,
        }
    }
}

impl Sum for EvidenceCounts {
    fn sum<Counts: Iterator<Item = EvidenceCounts>>(counts: Counts) -> EvidenceCounts {
        counts.fold(Self::default(), Add::add)
    }
}

impl fmt::Display for EvidenceCounts {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unverified {}, source {}, simulator {}, owner {}",
            self.unverified, self.source, self.simulator, self.owner
        )
    }
}
