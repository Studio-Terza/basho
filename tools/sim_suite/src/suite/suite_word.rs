use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct SuiteWord(String);

impl FromStr for SuiteWord {
    type Err = std::convert::Infallible;

    fn from_str(word: &str) -> Result<SuiteWord, Self::Err> {
        Ok(SuiteWord(word.trim().to_owned()))
    }
}
