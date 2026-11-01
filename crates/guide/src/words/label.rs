use crate::words::Spelling;

#[derive(
    newtype::Newtype,
    newtype::Display,
    derive_more::AsRef,
    derive_more::Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
#[as_ref(str)]
#[debug("{_0:?}")]
pub struct Label(&'static str);

impl Label {
    #[must_use]
    pub fn lowered(self) -> Spelling {
        Spelling::new(self.0.to_lowercase())
    }
}
