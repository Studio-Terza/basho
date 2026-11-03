use serde::Deserialize;

use crate::leg::Margin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Transfer {
    CrossGate,
    SameOperator,
}

impl From<Transfer> for Margin {
    fn from(transfer: Transfer) -> Margin {
        match transfer {
            Transfer::CrossGate => Margin::CROSS_GATE_TRANSFER,
            Transfer::SameOperator => Margin::SAME_OPERATOR_TRANSFER,
        }
    }
}
