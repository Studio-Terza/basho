use newtype::{Arbitrary, Newtype};

#[derive(
    newtype::Display,
    Newtype,
    Arbitrary,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct GoalCount(usize);
