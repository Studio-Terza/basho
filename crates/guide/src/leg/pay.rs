use serde::Deserialize;

use crate::leg::Margin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pay {
    StoredValue,
    Held,
    Buy,
    Pickup,
    OnBoard,
}

impl From<Pay> for Margin {
    fn from(pay: Pay) -> Margin {
        match pay {
            Pay::StoredValue | Pay::Held => Margin::TICKET_READY,
            Pay::Buy => Margin::TICKET_TO_BUY,
            Pay::Pickup | Pay::OnBoard => Margin::AT_THE_STOP,
        }
    }
}
