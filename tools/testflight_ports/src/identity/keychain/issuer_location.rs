use std::ffi::OsStr;
use std::str::FromStr;

use crate::failure::Failure;

pub struct IssuerLocation(String);

impl IssuerLocation {
    const ISSUER_MARKER: &str = "CA Issuers - URI:";
}

impl FromStr for IssuerLocation {
    type Err = Failure;

    fn from_str(extension: &str) -> Result<IssuerLocation, Failure> {
        extension
            .lines()
            .find_map(|line| line.trim().strip_prefix(Self::ISSUER_MARKER))
            .map(|location| IssuerLocation(location.to_owned()))
            .ok_or_else(|| Failure::Signing("the certificate names no issuer".to_owned()))
    }
}

impl AsRef<OsStr> for IssuerLocation {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(&self.0)
    }
}
