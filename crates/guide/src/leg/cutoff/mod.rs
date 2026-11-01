mod cutoff_kind;

use serde::Deserialize;

use crate::clock::ClockTime;
use crate::place::RubyText;
use crate::times::ListedTime;
use cutoff_kind::CutoffKind;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cutoff {
    kind: CutoffKind,
    at: ListedTime,
    name: Option<RubyText>,
}

impl Cutoff {
    #[must_use]
    pub fn kind(&self) -> CutoffKind {
        self.kind
    }

    #[must_use]
    pub fn deadline(&self) -> ClockTime {
        ClockTime::from(self.listed_at())
    }

    #[must_use]
    pub fn listed_at(&self) -> ListedTime {
        self.at
    }

    #[must_use]
    pub fn name(&self) -> Option<&RubyText> {
        self.name.as_ref()
    }
}
