use serde::Deserialize;

use crate::content::Content;
use crate::leg::LegIndex;
use crate::times::PlanTime;

#[derive(derive_more::AsRef, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoopSight {
    leg: LegIndex,
    name: Content,
    at: PlanTime,
}
