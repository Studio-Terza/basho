use crate::content::Content;
use serde::Deserialize;

#[derive(derive_more::AsRef, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Disagreement {
    #[as_ref]
    topic: Content,
    detail: Content,
}

impl Disagreement {
    #[must_use]
    pub fn detail(&self) -> &Content {
        &self.detail
    }
}
