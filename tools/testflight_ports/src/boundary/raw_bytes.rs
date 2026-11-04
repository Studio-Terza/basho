use base64::DecodeError;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::Deserialize;

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct RawBytes(Vec<u8>);

impl RawBytes {
    pub fn decoded(encoded: &str) -> Result<RawBytes, DecodeError> {
        STANDARD.decode(encoded).map(RawBytes)
    }

    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<String> for RawBytes {
    type Error = DecodeError;

    fn try_from(encoded: String) -> Result<RawBytes, DecodeError> {
        RawBytes::decoded(&encoded)
    }
}

impl From<Vec<u8>> for RawBytes {
    fn from(bytes: Vec<u8>) -> RawBytes {
        RawBytes(bytes)
    }
}

impl AsRef<[u8]> for RawBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
