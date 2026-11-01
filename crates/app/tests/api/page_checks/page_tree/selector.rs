use std::convert::Infallible;
use std::str::FromStr;

use super::compound::Compound;

pub struct Selector {
    compounds: Vec<Compound>,
}

impl Selector {
    const LIST_GAP: &str = ", ";
    const LIST: &str = ",";

    pub fn compounds(&self) -> &[Compound] {
        &self.compounds
    }
}

impl FromStr for Selector {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Selector, Infallible> {
        Ok(Selector {
            compounds: text
                .replace(Self::LIST_GAP, Self::LIST)
                .split_whitespace()
                .filter_map(|compound| compound.parse().ok())
                .collect(),
        })
    }
}
