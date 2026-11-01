use serde::Deserialize;

use crate::evidence::Evidence;
use crate::words::Label;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(try_from = "Evidence")]
pub struct Backing(Evidence);

impl TryFrom<Evidence> for Backing {
    type Error = Label;

    fn try_from(evidence: Evidence) -> Result<Backing, Label> {
        evidence
            .backing()
            .is_some()
            .then_some(())
            .ok_or_else(|| Label::new("this fact needs what backs it, not a guess"))
            .map(|()| Backing(evidence))
    }
}
