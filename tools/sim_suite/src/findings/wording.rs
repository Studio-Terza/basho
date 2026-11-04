#[derive(newtype::Newtype, Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct Wording(&'static str);
