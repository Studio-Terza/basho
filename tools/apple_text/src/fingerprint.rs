use std::ffi::OsStr;
use std::str::FromStr;

use crate::text_failure::{TextFailure, Written};

#[derive(newtype::Display, Clone, Debug, PartialEq, Eq)]
pub struct Fingerprint(String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Listing(String);

impl From<String> for Listing {
    fn from(text: String) -> Listing {
        Listing(text)
    }
}

impl Fingerprint {
    const HASH_LENGTH: usize = 40;

    #[must_use]
    pub fn listed_in(&self, listing: &Listing) -> bool {
        listing.0.contains(&self.0)
    }
}

impl From<&[u8]> for Fingerprint {
    fn from(der: &[u8]) -> Fingerprint {
        Fingerprint(
            sha1_smol::Sha1::from(der)
                .digest()
                .to_string()
                .to_uppercase(),
        )
    }
}

impl FromStr for Fingerprint {
    type Err = TextFailure;

    fn from_str(hash: &str) -> Result<Fingerprint, TextFailure> {
        (hash.len() == Self::HASH_LENGTH)
            .then(|| Fingerprint(hash.to_owned()))
            .ok_or_else(|| TextFailure::NotFingerprint(Written::new(hash.to_owned())))
    }
}

impl AsRef<OsStr> for Fingerprint {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(&self.0)
    }
}
