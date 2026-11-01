use super::violation::Violation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unfinished {
    Unloaded,
    Unchecked,
}

impl Unfinished {
    pub fn violation(self) -> Violation {
        match self {
            Unfinished::Unloaded => {
                Violation::new("unloaded", "Chrome did not load the page in three tries")
            }
            Unfinished::Unchecked => {
                Violation::new("unchecked", "the check script did not return a result")
            }
        }
    }
}
