use crate::task::gate::{Outcome, Tree};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posting {
    Post,
    WithheldDirtyTree,
    WithheldFailure,
}

impl Posting {
    #[must_use]
    pub fn decide(tree: Tree, outcome: &Outcome) -> Posting {
        match (outcome, tree) {
            (Outcome::Failed(_), _) => Posting::WithheldFailure,
            (Outcome::Passed, Tree::Dirty) => Posting::WithheldDirtyTree,
            (Outcome::Passed, Tree::Clean) => Posting::Post,
        }
    }
}
