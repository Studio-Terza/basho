#[derive(newtype::Newtype, newtype::Arbitrary, newtype::Display, Clone, Debug, PartialEq, Eq)]
pub struct ToolName(String);
