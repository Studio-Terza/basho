use std::fmt;

use guide::words::Spelling;
use serde::Deserialize;

use crate::key_entry::KeyEntry;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PickedMark {
    kind: KeyEntry,
    name: Option<Spelling>,
}

impl PickedMark {
    #[must_use]
    pub fn new(kind: KeyEntry, name: Option<Spelling>) -> PickedMark {
        PickedMark { kind, name }
    }
}

impl fmt::Display for PickedMark {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.kind.meaning())?;
        self.name
            .as_ref()
            .map_or(Ok(()), |name| write!(formatter, " · {name}"))
    }
}
