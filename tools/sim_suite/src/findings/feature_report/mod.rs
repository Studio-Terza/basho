mod unsimulated;

use crate::suite::{Feature, Standing, SuiteWord};
use unsimulated::Unsimulated;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct FeatureReport {
    feature: Feature,
    standing: Standing,
    passed: Vec<SuiteWord>,
    failed: Vec<SuiteWord>,
    unsimulated: Vec<Unsimulated>,
}

impl FeatureReport {
    #[must_use]
    pub fn new(feature: Feature, passed: Vec<SuiteWord>, failed: Vec<SuiteWord>) -> FeatureReport {
        FeatureReport {
            feature,
            standing: if passed.is_empty() || !failed.is_empty() {
                Standing::Failed
            } else {
                Standing::Passed
            },
            unsimulated: Unsimulated::listed_for(feature),
            passed,
            failed,
        }
    }

    #[must_use]
    pub fn standing(&self) -> Standing {
        self.standing
    }

    #[must_use]
    pub fn feature(&self) -> Feature {
        self.feature
    }
}
