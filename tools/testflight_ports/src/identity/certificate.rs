use serde::Deserialize;

use crate::bits::RawBytes;
use apple_text::Fingerprint;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    certificate_content: RawBytes,
}

impl Certificate {
    pub fn der(&self) -> &RawBytes {
        &self.certificate_content
    }

    pub fn fingerprint(&self) -> Fingerprint {
        Fingerprint::from(self.certificate_content.as_slice())
    }
}
