mod headsign;
mod line_name;
mod platform;
mod train_kind;
mod train_text_error;

use serde::Deserialize;

use crate::evidence::Evidence;

pub use headsign::Headsign;
pub use line_name::LineName;
pub use platform::Platform;
pub use train_kind::TrainKind;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Train {
    #[as_ref]
    platform: Platform,
    #[as_ref]
    headsign: Headsign,
    #[as_ref]
    line: LineName,
    #[as_ref]
    kind: TrainKind,
    #[as_ref]
    evidence: Evidence,
}
