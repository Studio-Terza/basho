mod referent;
pub use referent::Referent;
use serde::Deserialize;

use super::ReasonKind;
use crate::evidence::Evidence;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum RouteReason {
    NoneRecorded,
    Recorded {
        kind: ReasonKind,
        refers: Referent,
        evidence: Evidence,
    },
}

impl RouteReason {
    #[must_use]
    pub fn kind(&self) -> Option<ReasonKind> {
        match self {
            RouteReason::Recorded { kind, .. } => Some(*kind),
            RouteReason::NoneRecorded => None,
        }
    }

    #[must_use]
    pub fn referent(&self) -> Option<&Referent> {
        match self {
            RouteReason::Recorded { refers, .. } => Some(refers),
            RouteReason::NoneRecorded => None,
        }
    }

    #[must_use]
    pub fn evidence(&self) -> Option<&Evidence> {
        match self {
            RouteReason::Recorded { evidence, .. } => Some(evidence),
            RouteReason::NoneRecorded => None,
        }
    }
}
