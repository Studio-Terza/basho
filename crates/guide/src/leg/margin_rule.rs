use serde::Deserialize;

use crate::leg::margin::Margin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginRule {
    BoatCheckIn,
}

impl From<MarginRule> for Margin {
    fn from(margin_rule: MarginRule) -> Margin {
        match margin_rule {
            MarginRule::BoatCheckIn => Margin::BOAT_CHECK_IN,
        }
    }
}
