pub mod catch_note;
use serde::{Deserialize, Deserializer};

use crate::leg::LegIndex;
use catch_note::CatchNote;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegCatch {
    #[serde(deserialize_with = "LegCatch::zero_based")]
    index: LegIndex,
    catch: Vec<CatchNote>,
}

impl LegCatch {
    fn zero_based<'de, D: Deserializer<'de>>(deserializer: D) -> Result<LegIndex, D::Error> {
        usize::deserialize(deserializer).map(LegIndex::new)
    }

    #[must_use]
    pub fn notes(&self) -> &[CatchNote] {
        &self.catch
    }

    #[must_use]
    pub fn notes_for(&self, leg: LegIndex) -> &[CatchNote] {
        if leg == self.index { &self.catch } else { &[] }
    }
}
