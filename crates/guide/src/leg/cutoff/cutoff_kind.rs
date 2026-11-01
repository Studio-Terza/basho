use serde::Deserialize;

use crate::timeline::DeadlineKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CutoffKind {
    Closes,
    Entry,
    LastOrder,
}

impl CutoffKind {
    #[must_use]
    pub fn deadline_kind(self) -> DeadlineKind {
        match self {
            CutoffKind::Closes => DeadlineKind::Closes,
            CutoffKind::Entry => DeadlineKind::Entry,
            CutoffKind::LastOrder => DeadlineKind::LastOrder,
        }
    }
}
