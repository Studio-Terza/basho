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
        if usize::from(leg) == self.index {
            &self.catch
        } else {
            &[]
        }
    }
}
