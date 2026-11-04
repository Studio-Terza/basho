use newtype::Newtype;
use std::cmp::Ordering;

use crate::geo::distance::Distance;

#[derive(
    derive_more::Into,
    Newtype,
    newtype::Arbitrary,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct OffRoute(Distance);

impl PartialEq<Distance> for OffRoute {
    fn eq(&self, other: &Distance) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<Distance> for OffRoute {
    fn partial_cmp(&self, other: &Distance) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}
