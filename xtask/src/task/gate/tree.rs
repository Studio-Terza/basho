use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Tree {
    Clean,
    Dirty,
}

impl FromStr for Tree {
    type Err = Infallible;

    fn from_str(porcelain: &str) -> Result<Tree, Infallible> {
        Ok(if porcelain.trim().is_empty() {
            Tree::Clean
        } else {
            Tree::Dirty
        })
    }
}
