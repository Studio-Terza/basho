use serde::Deserialize;
use strum::IntoStaticStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, IntoStaticStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum StopKind {
    Landmark,
    Food,
    Shop,
    Sight,
    Service,
}
