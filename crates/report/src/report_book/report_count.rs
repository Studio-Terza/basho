#[derive(newtype::Newtype, newtype::Display, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReportCount(usize);
