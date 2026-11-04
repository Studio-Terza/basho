#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posting {
    Post,
    WithheldDirtyTree,
    WithheldFailure,
}
