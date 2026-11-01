use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct Detail(String);

impl<Problem: Error> From<&Problem> for Detail {
    fn from(problem: &Problem) -> Detail {
        Detail(problem.to_string())
    }
}

impl fmt::Display for Detail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
