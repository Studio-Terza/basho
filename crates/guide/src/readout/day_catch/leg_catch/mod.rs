pub mod catch_note;
use serde::Deserialize;

use crate::leg::LegIndex;
use catch_note::CatchNote;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegCatch {
    index: usize,
    catch: Vec<CatchNote>,
}

impl LegCatch {
    #[must_use]
    pub fn notes(&self) -> &[CatchNote] {
        &self.catch
    }

    #[must_use]
    pub fn notes_for(&self, leg: LegIndex) -> &[CatchNote] {
        Some(self.index)
            .filter(|index| *index == usize::from(leg))
            .map_or(&[], |_| self.catch.as_slice())
    }
}
