use std::fmt;

use crate::{ReportCount, ReportStore, ReportText};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Notice {
    Exported(ReportCount),
    Trouble(ReportText),
}

impl fmt::Display for Notice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Notice::Exported(count) => write!(
                formatter,
                "Exported {count}. Open Files, then On My iPhone, then Basho Walk: {}.",
                ReportStore::EXPORT_NAME
            ),
            Notice::Trouble(trouble) => write!(formatter, "PROBLEM: {trouble}"),
        }
    }
}
