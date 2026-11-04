use std::convert::Infallible;
use std::str::FromStr;

use crate::task::gate::Posting;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Tree {
    Clean,
    Dirty,
}

impl Tree {
    #[must_use]
    pub fn posting(self) -> Posting {
        match self {
            Tree::Clean => Posting::Post,
            Tree::Dirty => Posting::WithheldDirtyTree,
        }
    }
}

impl FromStr for Tree {
    type Err = Infallible;

    fn from_str(porcelain: &str) -> Result<Tree, Infallible> {
        Ok(porcelain
            .trim()
            .chars()
            .next()
            .map_or(Tree::Clean, |_| Tree::Dirty))
    }
}
