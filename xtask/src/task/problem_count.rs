#[derive(newtype::Display, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProblemCount(usize);

impl<Problem> From<&[Problem]> for ProblemCount {
    fn from(problems: &[Problem]) -> ProblemCount {
        ProblemCount(problems.len())
    }
}
