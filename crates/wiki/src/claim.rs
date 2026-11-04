use std::cmp::Ordering;

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
        match other.kind.cmp(&self.kind) {
            Ordering::Less => other,
            Ordering::Equal => Claim {
                entry: self.entry.filter(|entry| other.entry == Some(*entry)),
                ..self
            },
            Ordering::Greater => self,
        }
    }
}
