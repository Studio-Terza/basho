use std::str::FromStr;

use crate::suite::{Feature, SuiteWord};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct TestName {
    feature: Feature,
    case: SuiteWord,
}

impl TestName {
    const BUNDLE: &str = "BashoUITests.";

    #[must_use]
    pub fn feature(&self) -> Feature {
        self.feature
    }

    #[must_use]
    pub fn case(&self) -> &SuiteWord {
        &self.case
    }
}

impl FromStr for TestName {
    type Err = strum::ParseError;

    fn from_str(name: &str) -> Result<TestName, strum::ParseError> {
        let (class, case) = name
            .strip_prefix(Self::BUNDLE)
            .unwrap_or(name)
            .split_once(' ')
            .ok_or(strum::ParseError::VariantNotFound)?;
        Ok(TestName {
            feature: class.parse()?,
            case: case
                .parse()
                .map_err(|_: std::convert::Infallible| strum::ParseError::VariantNotFound)?,
        })
    }
}
