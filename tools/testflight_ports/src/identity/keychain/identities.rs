use std::convert::Infallible;
use std::str::FromStr;

use apple_text::Fingerprint;

pub struct Identities(Vec<Fingerprint>);

impl Identities {
    pub fn contains(&self, fingerprint: &Fingerprint) -> bool {
        self.0.contains(fingerprint)
    }
}

impl FromStr for Identities {
    type Err = Infallible;

    fn from_str(listing: &str) -> Result<Identities, Infallible> {
        Ok(Identities(
            listing
                .lines()
                .filter_map(|line| line.split_whitespace().nth(1))
                .filter_map(|hash| hash.parse().ok())
                .collect(),
        ))
    }
}
