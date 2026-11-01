mod counting_visitor;

use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::evidence::EvidenceCounts;
use counting_visitor::CountingVisitor;

#[derive(derive_more::From, derive_more::Into, Clone, Copy, Debug)]
pub struct EvidenceTally(EvidenceCounts);

impl<'de> Deserialize<'de> for EvidenceTally {
    fn deserialize<Input: Deserializer<'de>>(input: Input) -> Result<Self, Input::Error> {
        input.deserialize_any(CountingVisitor).map(EvidenceTally)
    }
}

impl FromStr for EvidenceTally {
    type Err = toml::de::Error;

    fn from_str(text: &str) -> Result<EvidenceTally, toml::de::Error> {
        toml::from_str(text)
    }
}
