use std::error::Error;

use crate::task::gate::Detail;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Passed,
    Failed(Detail),
}

impl<Passed, Problem: Error> From<&Result<Passed, Problem>> for Outcome {
    fn from(result: &Result<Passed, Problem>) -> Outcome {
        result.as_ref().map_or_else(
            |problem| Outcome::Failed(Detail::from(problem)),
            |_| Outcome::Passed,
        )
    }
}
