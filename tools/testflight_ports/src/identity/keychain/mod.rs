mod identities;
mod issuer_location;

use std::fs;
use std::path::Path;

use crate::failure::Failure;
use crate::staging::Staging;
use crate::tool::Tool;
use apple_text::{Fingerprint, Listing};
pub use identities::Identities;
use issuer_location::IssuerLocation;

pub struct Keychain;

impl Keychain {
    const ISSUER_FILE: &str = "issuer.cer";
    const AGENT: &str = "basho-trip-planner";

    pub fn identities() -> Result<Identities, Failure> {
        let listing = Tool::named("security")
            .args(["find-identity", "-v", "-p", "codesigning"])
            .text()?;
        Ok(listing.parse().unwrap_or_else(|never| match never {}))
    }

    pub fn import(path: &Path) -> Result<(), Failure> {
        Tool::named("security")
            .arg("import")
            .arg(path)
            .args(["-T", "/usr/bin/codesign"])
            .output()
            .map(drop)
    }

    pub fn trust_issuer_of(certificate: &Path, staging: &Staging) -> Result<(), Failure> {
        let location: IssuerLocation = Tool::named("openssl")
            .args([
                "x509",
                "-inform",
                "der",
                "-noout",
                "-ext",
                "authorityInfoAccess",
                "-in",
            ])
            .arg(certificate)
            .text()?
            .parse()?;
        let issuer = staging.path(Self::ISSUER_FILE);
        Tool::named("curl")
            .args(["--silent", "--show-error", "--fail", "--location"])
            .args(["--user-agent", Self::AGENT, "--output"])
            .arg(&issuer)
            .arg(location)
            .output()?;
        let der = fs::read(&issuer).map_err(Staging::io_failure(&issuer))?;
        let fingerprint = Fingerprint::from(der.as_slice());
        let installed = Tool::named("security")
            .args(["find-certificate", "-a", "-Z"])
            .text()?;
        if fingerprint.listed_in(&Listing::from(installed)) {
            Ok(())
        } else {
            Self::import(&issuer)
        }
    }
}
