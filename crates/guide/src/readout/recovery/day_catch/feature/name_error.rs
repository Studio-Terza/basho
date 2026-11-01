use crate::readout::recovery::catch_words::FeatureId;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("catching feature {0} has no name the walker can read")]
pub struct NameError(FeatureId);

impl From<FeatureId> for NameError {
    fn from(feature: FeatureId) -> NameError {
        NameError(feature)
    }
}
