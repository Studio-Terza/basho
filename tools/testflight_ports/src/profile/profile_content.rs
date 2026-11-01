use serde::Deserialize;

use crate::bits::RawBytes;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileContent {
    profile_content: RawBytes,
}

impl ProfileContent {
    pub fn decoded(&self) -> &RawBytes {
        &self.profile_content
    }
}
