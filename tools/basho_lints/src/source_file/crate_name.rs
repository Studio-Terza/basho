#[derive(newtype::Newtype, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CrateName<'file>(&'file str);
