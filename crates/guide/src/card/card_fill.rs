use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum CardFill {
    #[serde(rename = "inn")]
    TonightsInn,
}
