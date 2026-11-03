use strum::IntoEnumIterator;

use crate::findings::feature_report::FeatureReport;
use crate::suite::{Feature, Standing, SuiteWord, TestName};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TestTally {
    heard: Vec<HeardTest>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct HeardTest {
    test: TestName,
    standing: Standing,
}

impl TestTally {
    #[must_use]
    pub fn heard(mut self, test: TestName, standing: Standing) -> TestTally {
        self.heard.push(HeardTest { test, standing });
        self
    }

    fn cases(&self, feature: Feature, standing: Standing) -> Vec<SuiteWord> {
        self.heard
            .iter()
            .filter(|heard| heard.test.feature() == feature && heard.standing == standing)
            .map(|heard| heard.test.case().clone())
            .collect()
    }

    #[must_use]
    pub fn reports(&self) -> Vec<FeatureReport> {
        Feature::iter()
            .map(|feature| {
                FeatureReport::new(
                    feature,
                    self.cases(feature, Standing::Passed),
                    self.cases(feature, Standing::Failed),
                )
            })
            .collect()
    }
}
