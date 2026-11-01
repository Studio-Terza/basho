use newtype::Newtype;
#[derive(
    derive_more::Into, Newtype, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct StretchIndex(usize);
