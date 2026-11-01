use std::path::Path;

#[derive(
    newtype::Newtype,
    newtype::Display,
    derive_more::Deref,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[deref(forward)]
pub struct RelativePath(String);

impl RelativePath {
    #[must_use]
    pub fn under(root: &Path, path: &Path) -> RelativePath {
        RelativePath(
            path.strip_prefix(root)
                .unwrap_or(path)
                .to_string_lossy()
                .into_owned(),
        )
    }
}
