mod referent_error;

use serde::Deserialize;

use crate::content::{Content, NameKind, Part, TypedPart};
use referent_error::ReferentError;

#[derive(derive_more::AsRef, Clone, Debug, Deserialize)]
#[serde(try_from = "TypedPart")]
pub struct Referent {
    #[as_ref]
    content: Content,
}

impl TryFrom<TypedPart> for Referent {
    type Error = ReferentError;

    fn try_from(part: TypedPart) -> Result<Referent, ReferentError> {
        [NameKind::Entry, NameKind::Spot]
            .into_iter()
            .any(|kind| part.named(kind).is_some())
            .then(|| Referent {
                content: Content::from(vec![Part::Typed(part)]),
            })
            .ok_or(ReferentError::NotAName)
    }
}
