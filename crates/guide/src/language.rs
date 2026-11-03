use crate::classed::Classed;
use serde::Deserialize;

#[derive(strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[strum(serialize = "ja")]
    Ja,
    #[strum(serialize = "en")]
    En,
}

impl Classed for Language {}
