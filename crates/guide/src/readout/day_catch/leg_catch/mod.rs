pub mod catch_note;
use serde::{Deserialize, Deserializer};

use crate::leg::LegIndex;
use catch_note::CatchNote;

#[derive(derive_more::AsRef, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegCatch {
    #[as_ref]
    #[serde(deserialize_with = "LegCatch::zero_based")]
    index: LegIndex,
    catch: Vec<CatchNote>,
}

impl LegCatch {
    fn zero_based<'de, Input: Deserializer<'de>>(input: Input) -> Result<LegIndex, Input::Error> {
        usize::deserialize(input).map(LegIndex::new)
    }

    #[must_use]
    pub fn notes(&self) -> &[CatchNote] {
        &self.catch
    }
}
