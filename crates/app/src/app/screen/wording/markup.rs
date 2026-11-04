#[derive(
    newtype::Newtype,
    newtype::Display,
    derive_more::AsRef,
    derive_more::Deref,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[as_ref(str)]
#[deref(forward)]
pub struct Markup(String);
