use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Revision(u32);

impl Revision {
    const PREFIX: &str = "chromium-";
}

impl FromStr for Revision {
    type Err = ParseIntError;

    fn from_str(name: &str) -> Result<Revision, ParseIntError> {
        name.strip_prefix(Self::PREFIX)
            .unwrap_or_default()
            .parse::<u32>()
            .map(Revision)
    }
}
