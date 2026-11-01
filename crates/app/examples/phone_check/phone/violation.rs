use std::fmt;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Violation {
    kind: String,
    detail: String,
}

impl Violation {
    pub fn new(kind: &str, detail: &str) -> Violation {
        Violation {
            kind: kind.to_owned(),
            detail: detail.to_owned(),
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.kind, self.detail)
    }
}
