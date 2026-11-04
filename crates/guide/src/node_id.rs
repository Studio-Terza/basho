use newtype::Newtype;
use serde::Deserialize;

#[derive(
    newtype::Display,
    Newtype,
    newtype::Arbitrary,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
)]
pub struct NodeId(u64);
