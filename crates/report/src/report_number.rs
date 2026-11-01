use serde::{Deserialize, Serialize};

#[derive(
    newtype::Newtype,
    newtype::Display,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
)]
#[serde(transparent)]
pub struct ReportNumber(usize);
