use tool_text::ToolOutput;
mod certificate;
mod keychain;
mod signing_request;

use serde_json::json;

use crate::connect::{Connect, Document, Endpoint, Resource};
use crate::failure::Failure;
use crate::staging::Staging;
use apple_text::Fingerprint;
use certificate::Certificate;
use keychain::Keychain;
use signing_request::SigningRequest;

pub struct Identity {
    hash: Fingerprint,
    certificate: String,
}

impl Identity {
    const CERTIFICATE_FILE: &str = "distribution.cer";

    pub fn ensure(connect: &Connect, staging: &Staging) -> Result<Identity, Failure> {
        match Self::existing(connect)? {
            Some(identity) => Ok(identity),
            None => Self::enroll(connect, staging)
                .and_then(|()| Self::existing(connect))?
                .ok_or_else(|| {
                    Failure::Signing(ToolOutput::new("the new distribution certificate is not a valid signing identity in the keychain".to_owned()))
                }),
        }
    }

    #[must_use]
    pub fn hash(&self) -> &Fingerprint {
        &self.hash
    }

    #[must_use]
    pub fn certificate(&self) -> &str {
        &self.certificate
    }

    fn existing(connect: &Connect) -> Result<Option<Identity>, Failure> {
        let valid = Keychain::identities()?;
        let listing: Document<Vec<Resource<Certificate>>> =
            connect.fetch(&Endpoint::DistributionCertificates)?;
        let identities = listing
            .into_content()
            .iter()
            .map(|resource| Identity {
                hash: resource.attributes().fingerprint(),
                certificate: resource.identifier().as_ref().to_owned(),
            })
            .collect::<Vec<Identity>>();
        Ok(identities
            .into_iter()
            .find(|identity| valid.contains(&identity.hash)))
    }

    fn enroll(connect: &Connect, staging: &Staging) -> Result<(), Failure> {
        let request = SigningRequest::generate(staging)?;
        Keychain::import(request.key())?;
        let body = json!({"data": {"type": "certificates", "attributes": {
            "certificateType": "DISTRIBUTION",
            "csrContent": request.content(),
        }}});
        let created: Document<Resource<Certificate>> =
            connect.post(&Endpoint::Certificates, &body)?;
        let certificate = staging.write(
            Self::CERTIFICATE_FILE,
            created.into_content().attributes().der(),
        )?;
        Keychain::trust_issuer_of(&certificate, staging)?;
        Keychain::import(&certificate)?;
        println!(
            "testflight: created an Apple Distribution certificate and added it to the login keychain"
        );
        request.discard()
    }
}
