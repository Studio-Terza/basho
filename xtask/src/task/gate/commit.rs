use std::fmt;
use std::path::Path;
use std::str::FromStr;

use crate::task::failure::Failure;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct Commit(String);

impl Commit {
    const FULL_LENGTH: usize = 40;
}

impl FromStr for Commit {
    type Err = Failure;

    fn from_str(text: &str) -> Result<Commit, Failure> {
        let text = text.trim();
        (text.len() == Self::FULL_LENGTH && text.chars().all(|letter| letter.is_ascii_hexdigit()))
            .then(|| Commit(text.to_ascii_lowercase()))
            .ok_or(Failure::NotACommit)
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl AsRef<Path> for Commit {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}
