use std::error::Error;

use crate::task::gate::{Detail, Posting, Tree};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Passed,
    Failed(Detail),
}

impl Outcome {
    #[must_use]
    pub fn posting(&self, tree: Tree) -> Posting {
        match self {
            Outcome::Failed(_) => Posting::WithheldFailure,
            Outcome::Passed => tree.posting(),
        }
    }
}

impl<Passed, Problem: Error> From<&Result<Passed, Problem>> for Outcome {
    fn from(result: &Result<Passed, Problem>) -> Outcome {
        result.as_ref().map_or_else(
            |problem| Outcome::Failed(Detail::from(problem)),
            |_| Outcome::Passed,
        )
    }
}
