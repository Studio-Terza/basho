use std::collections::BTreeMap;

use newtype::Newtype;

use crate::catch_words::FeatureName;

#[derive(Newtype, newtype::Arbitrary, derive_more::Deref, Clone, Debug, Default, PartialEq, Eq)]
pub struct Readings(BTreeMap<FeatureName, FeatureName>);
