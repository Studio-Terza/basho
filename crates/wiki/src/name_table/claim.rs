use crate::EntryId;
use crate::entry::EntryKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Claim {
    entry: Option<EntryId>,
    kind: EntryKind,
}

impl Claim {
    #[must_use]
    pub fn new(entry: EntryId, kind: EntryKind) -> Claim {
        Claim {
            entry: Some(entry),
            kind,
        }
    }

    #[must_use]
    pub fn entry(self) -> Option<EntryId> {
        self.entry
    }

    #[must_use]
    pub fn joined(self, other: Claim) -> Claim {
        if other.kind < self.kind {
            other
        } else if other.kind == self.kind && other.entry != self.entry {
            Claim {
                entry: None,
                ..self
            }
        } else {
            self
        }
    }
}
