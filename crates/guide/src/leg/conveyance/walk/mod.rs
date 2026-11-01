mod reason_kind;
mod referent;
mod route_reason;

use serde::Deserialize;

pub use reason_kind::ReasonKind;
use referent::Referent;
pub use route_reason::RouteReason;

#[derive(derive_more::AsRef, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Walk {
    #[as_ref]
    reason: RouteReason,
}
