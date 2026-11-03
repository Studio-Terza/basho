mod reason_kind;
mod route_reason;

use serde::Deserialize;

pub use reason_kind::ReasonKind;
pub use route_reason::RouteReason;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Walk {
    #[as_ref]
    reason: RouteReason,
}
